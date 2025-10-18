use openapiv3::{OpenAPI, PathItem, Schema, SchemaKind, Type as OpenApiType, ReferenceOr};
use std::collections::{HashMap, HashSet};
use convert_case::{Case, Casing};
use crate::mermaid_parser::MermaidErParser;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ResourceInfo {
    pub name: String,
    pub fields: Vec<FieldInfo>,
    pub operations: Vec<String>, // GET, POST, PUT, DELETE
    pub relationships: Vec<RelationshipInfo>, // ER diagram relationships
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct RelationshipInfo {
    pub from_table: String,
    pub to_table: String,
    pub relationship_type: RelationshipType,
    pub from_field: Option<String>,
    pub to_field: Option<String>,
    pub constraint: Option<String>, // CASCADE, RESTRICT, etc.
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum RelationshipType {
    OneToOne,
    OneToMany,
    ManyToOne,
    ManyToMany,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct FieldInfo {
    pub name: String,
    pub field_type: FieldType,
    pub required: bool,
    pub unique: bool, // ユニーク制約
    pub reference: Option<String>, // 参照先テーブル名
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum FieldType {
    // String types
    String,
    Text,
    Uuid,
    
    // Numeric types
    Integer,
    BigInt,
    SmallInt,
    Unsigned,
    BigUnsigned,
    SmallUnsigned,
    Float,
    Double,
    Decimal,
    DecimalLen,
    Money,
    
    // Other types
    Boolean,
    Date,
    DateTime,
    TimestampWithTimeZone,
    Json,
    JsonBinary,
    Blob,
    BinaryLen,
    VarBinary,
    Array,
    
    // References
    References(String), // 外部キー
}

pub struct OpenAPIAnalyzer {
    openapi: OpenAPI,
    er_relationships: Vec<RelationshipInfo>,
}

impl OpenAPIAnalyzer {
    pub fn new(openapi: OpenAPI) -> Self {
        Self { 
            openapi,
            er_relationships: Vec::new(),
        }
    }

    /// Create analyzer with ER diagram relationships
    pub fn with_er_diagram(openapi: OpenAPI, er_file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let parser = MermaidErParser::new();
        
        let er_relationships = if er_file_path.ends_with(".md") {
            parser.parse_markdown_file(er_file_path)?
        } else if er_file_path.ends_with(".mermaid") || er_file_path.ends_with(".mmd") {
            parser.parse_mermaid_file(er_file_path)?
        } else {
            return Err("Unsupported ER diagram file format. Use .md, .mermaid, or .mmd files".into());
        };

        Ok(Self {
            openapi,
            er_relationships,
        })
    }

    /// OpenAPI仕様からリソース情報を抽出
    pub fn analyze(&self) -> Result<Vec<ResourceInfo>, Box<dyn std::error::Error>> {
        let mut resources = HashMap::<String, ResourceInfo>::new();
        
        // パス解析: リソース名とオペレーション抽出
        for (path, path_item) in &self.openapi.paths.paths {
            if let ReferenceOr::Item(item) = path_item {
                let resource_name = self.extract_resource_name(path);
                
                let operations = self.extract_operations(item);
                
                let resource = resources.entry(resource_name.clone()).or_insert(ResourceInfo {
                    name: resource_name,
                    fields: Vec::new(),
                    operations: Vec::new(),
                    relationships: Vec::new(),
                });
                
                resource.operations.extend(operations);
            }
        }

        // スキーマ解析: フィールド情報抽出
        if let Some(components) = &self.openapi.components {
            for (schema_name, schema_ref) in &components.schemas {
                if let ReferenceOr::Item(schema) = schema_ref {
                    let fields = self.extract_fields(schema)?;
                    let schema_resource_name = schema_name.to_case(Case::Snake);
                    
                    // スキーマ名とマッチする既存リソースを探す
                    let matching_resource = self.find_matching_resource(&resources, &schema_resource_name);
                    
                    if let Some(resource_name) = matching_resource {
                        if let Some(resource) = resources.get_mut(&resource_name) {
                            resource.fields = fields;
                        }
                    } else {
                        // スキーマのみ定義されているリソース
                        resources.insert(schema_resource_name.clone(), ResourceInfo {
                            name: schema_resource_name,
                            fields,
                            operations: vec!["GET".to_string(), "POST".to_string()], // デフォルト
                            relationships: Vec::new(),
                        });
                    }
                }
            }
        }

        // ER図から関係性を統合
        self.integrate_er_relationships(&mut resources);

        Ok(resources.into_values().collect())
    }

    /// ER図の関係性をリソースに統合
    fn integrate_er_relationships(&self, resources: &mut HashMap<String, ResourceInfo>) {
        for er_rel in &self.er_relationships {
            // テーブル名を正規化してマッチング
            let from_resource_name = self.normalize_table_name(&er_rel.from_table);
            let to_resource_name = self.normalize_table_name(&er_rel.to_table);
            
            // From側のリソースに関係性を追加
            if let Some(resource) = resources.get_mut(&from_resource_name) {
                resource.relationships.push(er_rel.clone());
            }
            
            // 双方向の関係性の場合、逆向きも追加
            match er_rel.relationship_type {
                RelationshipType::OneToOne => {
                    if let Some(resource) = resources.get_mut(&to_resource_name) {
                        resource.relationships.push(RelationshipInfo {
                            from_table: er_rel.to_table.clone(),
                            to_table: er_rel.from_table.clone(),
                            relationship_type: RelationshipType::OneToOne,
                            from_field: er_rel.to_field.clone(),
                            to_field: er_rel.from_field.clone(),
                            constraint: er_rel.constraint.clone(),
                        });
                    }
                }
                RelationshipType::OneToMany => {
                    if let Some(resource) = resources.get_mut(&to_resource_name) {
                        resource.relationships.push(RelationshipInfo {
                            from_table: er_rel.to_table.clone(),
                            to_table: er_rel.from_table.clone(),
                            relationship_type: RelationshipType::ManyToOne,
                            from_field: er_rel.to_field.clone(),
                            to_field: er_rel.from_field.clone(),
                            constraint: er_rel.constraint.clone(),
                        });
                    }
                }
                RelationshipType::ManyToOne => {
                    if let Some(resource) = resources.get_mut(&to_resource_name) {
                        resource.relationships.push(RelationshipInfo {
                            from_table: er_rel.to_table.clone(),
                            to_table: er_rel.from_table.clone(),
                            relationship_type: RelationshipType::OneToMany,
                            from_field: er_rel.to_field.clone(),
                            to_field: er_rel.from_field.clone(),
                            constraint: er_rel.constraint.clone(),
                        });
                    }
                }
                RelationshipType::ManyToMany => {
                    if let Some(resource) = resources.get_mut(&to_resource_name) {
                        resource.relationships.push(RelationshipInfo {
                            from_table: er_rel.to_table.clone(),
                            to_table: er_rel.from_table.clone(),
                            relationship_type: RelationshipType::ManyToMany,
                            from_field: er_rel.to_field.clone(),
                            to_field: er_rel.from_field.clone(),
                            constraint: er_rel.constraint.clone(),
                        });
                    }
                }
            }
            
            // ER図の情報に基づいて外部キーフィールドを改善
            self.enhance_foreign_key_fields(resources, er_rel);
        }
    }

    /// テーブル名を正規化してリソース名とマッチング
    fn normalize_table_name(&self, table_name: &str) -> String {
        // 大文字小文字を統一し、単数形/複数形をチェック
        let normalized = table_name.to_case(Case::Snake);
        
        // 複数形を試す
        let plural = format!("{}s", normalized);
        plural
    }

    /// ER図の情報に基づいて外部キーフィールドを改善
    fn enhance_foreign_key_fields(&self, resources: &mut HashMap<String, ResourceInfo>, er_rel: &RelationshipInfo) {
        if let (Some(from_field), Some(to_field)) = (&er_rel.from_field, &er_rel.to_field) {
            let from_resource_name = self.normalize_table_name(&er_rel.from_table);
            
            if let Some(resource) = resources.get_mut(&from_resource_name) {
                // 既存のフィールドを探して情報を更新
                for field in &mut resource.fields {
                    if field.name == *from_field {
                        // 参照先テーブル情報を更新
                        field.reference = Some(er_rel.to_table.to_case(Case::Snake));
                        field.field_type = FieldType::References(er_rel.to_table.to_case(Case::Snake));
                        break;
                    }
                }
                
                // フィールドが存在しない場合は新規作成
                let field_exists = resource.fields.iter().any(|f| f.name == *from_field);
                if !field_exists {
                    resource.fields.push(crate::analyzer::FieldInfo {
                        name: from_field.clone(),
                        field_type: FieldType::References(er_rel.to_table.to_case(Case::Snake)),
                        required: true,
                        unique: false,
                        reference: Some(er_rel.to_table.to_case(Case::Snake)),
                    });
                }
            }
        }
    }

    /// パスからリソース名を抽出 (/users/{id} -> users)
    fn extract_resource_name(&self, path: &str) -> String {
        let path = path.trim_start_matches('/');
        let segments: Vec<&str> = path.split('/').collect();
        
        if segments.is_empty() {
            return "resource".to_string();
        }

        // 最初のセグメントをリソース名として使用 (パラメータ除去)
        let resource_segment = segments[0];
        if resource_segment.starts_with('{') && resource_segment.ends_with('}') {
            // ルートレベルパラメータの場合
            "resource".to_string()
        } else {
            resource_segment.to_case(Case::Snake)
        }
    }
    
    /// スキーマ名とマッチするリソースを探す (user -> users のような単数/複数変換を考慮)
    fn find_matching_resource(&self, resources: &HashMap<String, ResourceInfo>, schema_name: &str) -> Option<String> {
        // 完全一致を最初にチェック
        if resources.contains_key(schema_name) {
            return Some(schema_name.to_string());
        }
        
        // 複数形でのマッチをチェック (user -> users)
        let plural_name = format!("{}s", schema_name);
        if resources.contains_key(&plural_name) {
            return Some(plural_name);
        }
        
        // 単数形でのマッチをチェック (users -> user)
        if schema_name.ends_with('s') {
            let singular_name = &schema_name[..schema_name.len() - 1];
            if resources.contains_key(singular_name) {
                return Some(singular_name.to_string());
            }
        }
        
        None
    }

    /// PathItemからHTTPオペレーションを抽出
    fn extract_operations(&self, path_item: &PathItem) -> Vec<String> {
        let mut operations = Vec::new();
        
        if path_item.get.is_some() { operations.push("GET".to_string()); }
        if path_item.post.is_some() { operations.push("POST".to_string()); }
        if path_item.put.is_some() { operations.push("PUT".to_string()); }
        if path_item.delete.is_some() { operations.push("DELETE".to_string()); }
        if path_item.patch.is_some() { operations.push("PATCH".to_string()); }
        
        operations
    }

    /// スキーマからフィールド情報を抽出
    fn extract_fields(&self, schema: &Schema) -> Result<Vec<FieldInfo>, Box<dyn std::error::Error>> {
        let mut fields = Vec::new();
        
        if let SchemaKind::Type(openapiv3::Type::Object(object_type)) = &schema.schema_kind {
            let required_fields: HashSet<&String> = object_type.required.iter().collect();
            
            for (prop_name, prop_ref) in &object_type.properties {
                if let ReferenceOr::Item(prop_schema) = prop_ref {
                    let mut field_type = self.map_openapi_type_to_loco(&prop_schema.schema_kind);
                    
                    // フィールド名やスキーマ情報に基づく特殊なタイプ検出
                    field_type = self.enhance_field_type_detection(prop_name, field_type, prop_schema);
                    
                    let required = required_fields.contains(prop_name);
                    let unique = self.detect_unique_constraint(prop_name, prop_schema);
                    let reference = self.detect_reference(prop_name, &field_type);
                    
                    fields.push(FieldInfo {
                        name: prop_name.to_case(Case::Snake),
                        field_type: if reference.is_some() { 
                            FieldType::References(reference.clone().unwrap()) 
                        } else { 
                            field_type 
                        },
                        required,
                        unique,
                        reference,
                    });
                }
            }
        }
        
        Ok(fields)
    }

    /// OpenAPIタイプをLocoタイプにマッピング
    fn map_openapi_type_to_loco(&self, schema_kind: &SchemaKind) -> FieldType {
        match schema_kind {
            SchemaKind::Type(openapi_type) => match openapi_type {
                OpenApiType::String(string_type) => {
                    // フォーマット指定がある場合を最初にチェック
                    match &string_type.format {
                        openapiv3::VariantOrUnknownOrEmpty::Item(format) => match format {
                            openapiv3::StringFormat::DateTime => FieldType::DateTime,
                            openapiv3::StringFormat::Date => FieldType::Date,
                            _ => {
                                // 文字列長で判定
                                if let Some(max_length) = string_type.max_length {
                                    if max_length > 500 {
                                        FieldType::Text
                                    } else {
                                        FieldType::String
                                    }
                                } else {
                                    FieldType::String
                                }
                            },
                        },
                        _ => {
                            // フォーマット指定がない場合は文字列長で判定
                            if let Some(max_length) = string_type.max_length {
                                if max_length > 500 {
                                    FieldType::Text
                                } else {
                                    FieldType::String
                                }
                            } else {
                                FieldType::String
                            }
                        }
                    }
                }
                OpenApiType::Number(number_type) => {
                    // 数値フォーマットに基づく判定
                    match &number_type.format {
                        openapiv3::VariantOrUnknownOrEmpty::Item(format) => match format {
                            openapiv3::NumberFormat::Float => FieldType::Float,
                            openapiv3::NumberFormat::Double => FieldType::Double,
                        },
                        _ => FieldType::Decimal, // デフォルトはDecimal
                    }
                }
                OpenApiType::Integer(integer_type) => {
                    // 整数フォーマットに基づく判定
                    match &integer_type.format {
                        openapiv3::VariantOrUnknownOrEmpty::Item(format) => match format {
                            openapiv3::IntegerFormat::Int32 => FieldType::Integer,
                            openapiv3::IntegerFormat::Int64 => FieldType::BigInt,
                        },
                        _ => FieldType::Integer, // デフォルト
                    }
                }
                OpenApiType::Boolean(_) => FieldType::Boolean,
                OpenApiType::Array(_) => FieldType::Array,
                OpenApiType::Object(_) => FieldType::Json,
            },
            _ => FieldType::String,
        }
    }

    /// フィールド名やスキーマ情報に基づいてタイプを精緻化
    fn enhance_field_type_detection(&self, field_name: &str, field_type: FieldType, schema: &Schema) -> FieldType {
        let field_name_lower = field_name.to_lowercase();
        
        // UUIDの検出
        if field_name_lower.contains("uuid") || field_name_lower.contains("guid") {
            return FieldType::Uuid;
        }
        
        // IDフィールドのUUID判定
        if field_name_lower == "id" || field_name_lower.ends_with("_id") {
            // スキーマの説明や例でUUIDであるかチェック
            if let Some(description) = &schema.schema_data.description {
                if description.to_lowercase().contains("uuid") {
                    return FieldType::Uuid;
                }
            }
            if let Some(example) = &schema.schema_data.example {
                if let Some(example_str) = example.as_str() {
                    if example_str.len() == 36 && example_str.chars().filter(|&c| c == '-').count() == 4 {
                        return FieldType::Uuid;
                    }
                }
            }
        }
        
        // JSONフィールドの検出
        if field_name_lower.contains("json") || field_name_lower.contains("metadata") 
            || field_name_lower.contains("config") || field_name_lower.contains("settings") {
            match field_type {
                FieldType::String => FieldType::Json,
                _ => field_type,
            }
        } else {
            field_type
        }
    }

    /// ユニーク制約を検出
    fn detect_unique_constraint(&self, field_name: &str, schema: &Schema) -> bool {
        let field_name_lower = field_name.to_lowercase();
        
        // 一般的にユニークとして扱われるフィールド
        if field_name_lower == "email" 
            || field_name_lower == "username" 
            || field_name_lower == "slug" 
            || field_name_lower == "sku" 
            || field_name_lower.contains("unique") {
            return true;
        }
        
        // スキーマの説明からユニーク性を推定
        if let Some(description) = &schema.schema_data.description {
            let desc_lower = description.to_lowercase();
            if desc_lower.contains("unique") 
                || desc_lower.contains("distinct")
                || desc_lower.contains("one-of-a-kind") {
                return true;
            }
        }
        
        false
    }

    /// 参照関係を検出 (命名規則ベース)
    fn detect_reference(&self, field_name: &str, field_type: &FieldType) -> Option<String> {
        match field_type {
            FieldType::Integer | FieldType::Uuid => {
                // user_id, userId -> user への参照
                if field_name.ends_with("_id") {
                    let table_name = field_name.strip_suffix("_id").unwrap();
                    Some(table_name.to_string())
                } else if field_name.ends_with("Id") {
                    let table_name = field_name.strip_suffix("Id").unwrap().to_case(Case::Snake);
                    Some(table_name)
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_resource_name() {
        let openapi = OpenAPI::default();
        let analyzer = OpenAPIAnalyzer::new(openapi);
        
        assert_eq!(analyzer.extract_resource_name("/users"), "users");
        assert_eq!(analyzer.extract_resource_name("/users/{id}"), "users");
        assert_eq!(analyzer.extract_resource_name("/api/v1/posts"), "api");
    }

    #[test]
    fn test_reference_detection() {
        let openapi = OpenAPI::default();
        let analyzer = OpenAPIAnalyzer::new(openapi);
        
        assert_eq!(
            analyzer.detect_reference("user_id", &FieldType::Integer), 
            Some("user".to_string())
        );
        assert_eq!(
            analyzer.detect_reference("userId", &FieldType::Integer), 
            Some("user".to_string())
        );
        assert_eq!(
            analyzer.detect_reference("title", &FieldType::String), 
            None
        );
    }
}