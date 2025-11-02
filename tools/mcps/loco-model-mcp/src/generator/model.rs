// Model code generator - converts ModelSpecification to Loco model code

use crate::error::{LocoModelError, Result};
use crate::models::field::{FieldDefinition, FieldValidation, LocoFieldType};
use crate::models::relationship::RelationshipDefinition;
use crate::models::specification::ModelSpecification;
use crate::validation::parse::parse_model_spec;
use crate::validation::semantic::validate_model_semantics;
use crate::generator::templates::{
    FieldContext, HookContext, ModelTemplateContext, RelationshipContext, render_model,
};
use serde_json::Value;

/// Generate Loco model code from JSON specification
pub fn generate_model_code(json: &Value) -> Result<String> {
    // Parse and validate the specification
    let spec = parse_model_spec(json)?;
    validate_model_semantics(&spec)?;

    // Build template context
    let context = build_template_context(&spec)?;

    // Render the template
    render_model(&context)
}

/// Build template context from model specification
fn build_template_context(spec: &ModelSpecification) -> Result<ModelTemplateContext> {
    // Convert fields
    let fields = spec
        .fields
        .iter()
        .map(|field| build_field_context(field, spec))
        .collect::<Result<Vec<_>>>()?;

    // Convert relationships
    let relationships = spec.relationships
        .iter()
        .map(build_relationship_context)
        .collect::<Result<Vec<_>>>()?;

    // Convert hooks (empty for now, Phase 4)
    let hooks: Vec<HookContext> = vec![];

    // Detect feature flags
    let has_uuid = fields.iter().any(|f| f.rust_type == "Uuid");
    let has_datetime = fields.iter().any(|f| f.rust_type.contains("DateTime<Utc>"));
    let has_date = fields.iter().any(|f| f.rust_type.contains("NaiveDate"));
    let has_time = fields.iter().any(|f| f.rust_type.contains("NaiveTime"));
    let has_decimal = fields.iter().any(|f| f.rust_type.contains("Decimal"));
    let has_validations = fields.iter().any(|f| !f.validations.is_empty());

    // Determine table name
    let table_name = spec
        .table_name
        .clone()
        .unwrap_or_else(|| to_table_name(&spec.name));

    Ok(ModelTemplateContext {
        name: spec.name.clone(),
        table_name,
        description: None, // No description in ModelSpecification
        fields,
        relationships,
        hooks,
        has_uuid,
        has_datetime,
        has_date,
        has_time,
        has_decimal,
        has_validations,
    })
}

/// Build field context from field definition
fn build_field_context(
    field: &FieldDefinition,
    spec: &ModelSpecification,
) -> Result<FieldContext> {
    let rust_type = map_field_type(&field.field_type, field.nullable)?;

    let column_type = match &field.field_type {
        LocoFieldType::Decimal { precision, scale } => {
            Some(format!("Decimal(Some(({}, {})))", precision, scale))
        }
        _ => None,
    };

    let validations = field
        .validations
        .iter()
        .map(field_validation_to_attribute)
        .collect::<Result<Vec<_>>>()?;

    Ok(FieldContext {
        name: field.name.clone(),
        rust_type,
        column_type,
        description: field.description.clone(),
        primary_key: field.primary_key,
        unique: field.unique,
        indexed: field.indexed,
        validations,
    })
}

/// Build relationship context from relationship definition
fn build_relationship_context(rel: &RelationshipDefinition) -> Result<RelationshipContext> {
    use crate::models::relationship::RelationshipDefinition as RD;

    let (name, model, foreign_key, relationship_type) = match rel {
        RD::BelongsTo { name, model, foreign_key, .. } => {
            (name, model, foreign_key, "belongs_to")
        }
        RD::HasMany { name, model, foreign_key, .. } => {
            (name, model, foreign_key, "has_many")
        }
        RD::HasOne { name, model, foreign_key, .. } => {
            (name, model, foreign_key, "has_one")
        }
        RD::ManyToMany { name, model, .. } => {
            (name, model, &None, "many_to_many")
        }
    };

    let name_pascal = to_pascal_case(name);
    let model_snake = to_snake_case(model);
    let foreign_key_pascal = foreign_key.as_ref().map(|fk| to_pascal_case(fk));

    Ok(RelationshipContext {
        name_pascal,
        model_snake,
        relationship_type: relationship_type.to_string(),
        foreign_key_pascal,
    })
}

/// Map LocoFieldType to Rust type string
fn map_field_type(field_type: &LocoFieldType, nullable: bool) -> Result<String> {
    let base_type = match field_type {
        LocoFieldType::String => "String".to_string(),
        LocoFieldType::Text => "String".to_string(),
        LocoFieldType::Integer => "i32".to_string(),
        LocoFieldType::SmallInt => "i16".to_string(),
        LocoFieldType::BigInt => "i64".to_string(),
        LocoFieldType::Uuid => "Uuid".to_string(),
        LocoFieldType::Boolean => "bool".to_string(),
        LocoFieldType::Timestamp => "DateTime<Utc>".to_string(),
        LocoFieldType::Date => "NaiveDate".to_string(),
        LocoFieldType::Time => "NaiveTime".to_string(),
        LocoFieldType::Json => "serde_json::Value".to_string(),
        LocoFieldType::Jsonb => "serde_json::Value".to_string(),
        LocoFieldType::Float => "f32".to_string(),
        LocoFieldType::Double => "f64".to_string(),
        LocoFieldType::Binary => "Vec<u8>".to_string(),
        LocoFieldType::Decimal { .. } => "Decimal".to_string(),
        LocoFieldType::Reference { model, .. } => format!("{}Id", model),
    };

    if nullable {
        Ok(format!("Option<{}>", base_type))
    } else {
        Ok(base_type.to_string())
    }
}

/// Convert field validation to validator attribute
fn field_validation_to_attribute(validation: &FieldValidation) -> Result<String> {
    match validation.validation_type.as_str() {
        "email" => Ok("#[validate(email)]".to_string()),
        "url" => Ok("#[validate(url)]".to_string()),
        "length" => {
            if let Some(params) = &validation.params {
                let mut parts = vec![];
                if let Some(min) = params.get("min") {
                    if let Some(min_val) = min.as_u64() {
                        parts.push(format!("min = {}", min_val));
                    }
                }
                if let Some(max) = params.get("max") {
                    if let Some(max_val) = max.as_u64() {
                        parts.push(format!("max = {}", max_val));
                    }
                }
                if !parts.is_empty() {
                    Ok(format!("#[validate(length({}))]", parts.join(", ")))
                } else {
                    Ok("#[validate(length)]".to_string())
                }
            } else {
                Ok("#[validate(length)]".to_string())
            }
        }
        "regex" => {
            if let Some(params) = &validation.params {
                if let Some(pattern) = params.get("pattern").and_then(|p| p.as_str()) {
                    Ok(format!("#[validate(regex = \"{}\")]", pattern))
                } else {
                    Err(LocoModelError::Validation(
                        "Regex validation requires 'pattern' parameter".to_string(),
                    ))
                }
            } else {
                Err(LocoModelError::Validation(
                    "Regex validation requires parameters".to_string(),
                ))
            }
        }
        _ => Err(LocoModelError::Validation(format!(
            "Unknown validation type: {}",
            validation.validation_type
        ))),
    }
}

/// Convert string to snake_case
fn to_snake_case(s: &str) -> String {
    let mut result = String::new();

    for (i, ch) in s.chars().enumerate() {
        if ch.is_uppercase() {
            // Add underscore before uppercase letters (except at the start)
            if i > 0 {
                result.push('_');
            }
            result.push(ch.to_ascii_lowercase());
        } else {
            result.push(ch);
        }
    }

    result
}

/// Convert string to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => {
                    let rest: String = chars.map(|c| c.to_ascii_lowercase()).collect();
                    format!("{}{}", first.to_uppercase(), rest)
                }
                None => String::new(),
            }
        })
        .collect()
}

/// Convert model name to table name (pluralized snake_case)
fn to_table_name(model_name: &str) -> String {
    let snake = to_snake_case(model_name);

    // Simple pluralization rules
    if snake.ends_with('s') || snake.ends_with("ch") || snake.ends_with("sh") {
        format!("{}es", snake)
    } else if snake.ends_with('y') {
        format!("{}ies", &snake[..snake.len() - 1])
    } else {
        format!("{}s", snake)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_to_snake_case() {
        assert_eq!(to_snake_case("User"), "user");
        assert_eq!(to_snake_case("UserPost"), "user_post");
        assert_eq!(to_snake_case("HTTPResponse"), "h_t_t_p_response");
        assert_eq!(to_snake_case("already_snake"), "already_snake");
    }

    #[test]
    fn test_to_pascal_case() {
        assert_eq!(to_pascal_case("user"), "User");
        assert_eq!(to_pascal_case("user_post"), "UserPost");
        assert_eq!(to_pascal_case("http_response"), "HttpResponse");
        assert_eq!(to_pascal_case("AlreadyPascal"), "Alreadypascal");
    }

    #[test]
    fn test_to_table_name() {
        assert_eq!(to_table_name("User"), "users");
        assert_eq!(to_table_name("Post"), "posts");
        assert_eq!(to_table_name("Category"), "categories");
        assert_eq!(to_table_name("Address"), "addresses");
        assert_eq!(to_table_name("UserPost"), "user_posts");
    }

    #[test]
    fn test_map_field_type_basic() {
        assert_eq!(
            map_field_type(&LocoFieldType::String, false).unwrap(),
            "String"
        );
        assert_eq!(
            map_field_type(&LocoFieldType::Integer, false).unwrap(),
            "i32"
        );
        assert_eq!(
            map_field_type(&LocoFieldType::Boolean, false).unwrap(),
            "bool"
        );
        assert_eq!(
            map_field_type(&LocoFieldType::Uuid, false).unwrap(),
            "Uuid"
        );
    }

    #[test]
    fn test_map_field_type_nullable() {
        assert_eq!(
            map_field_type(&LocoFieldType::String, true).unwrap(),
            "Option<String>"
        );
        assert_eq!(
            map_field_type(&LocoFieldType::Integer, true).unwrap(),
            "Option<i32>"
        );
    }

    #[test]
    fn test_map_field_type_complex() {
        assert_eq!(
            map_field_type(&LocoFieldType::Timestamp, false).unwrap(),
            "DateTime<Utc>"
        );
        assert_eq!(
            map_field_type(&LocoFieldType::Date, false).unwrap(),
            "NaiveDate"
        );
        assert_eq!(
            map_field_type(&LocoFieldType::Decimal { precision: 10, scale: 2 }, false).unwrap(),
            "Decimal"
        );
    }

    #[test]
    fn test_field_validation_to_attribute() {
        use serde_json::json;

        assert_eq!(
            field_validation_to_attribute(&FieldValidation {
                validation_type: "email".to_string(),
                params: None,
            })
            .unwrap(),
            "#[validate(email)]"
        );

        assert_eq!(
            field_validation_to_attribute(&FieldValidation {
                validation_type: "length".to_string(),
                params: Some(json!({"min": 3, "max": 50})),
            })
            .unwrap(),
            "#[validate(length(min = 3, max = 50))]"
        );

        assert_eq!(
            field_validation_to_attribute(&FieldValidation {
                validation_type: "regex".to_string(),
                params: Some(json!({"pattern": "^[a-z]+$"})),
            })
            .unwrap(),
            "#[validate(regex = \"^[a-z]+$\")]"
        );

        assert_eq!(
            field_validation_to_attribute(&FieldValidation {
                validation_type: "url".to_string(),
                params: None,
            })
            .unwrap(),
            "#[validate(url)]"
        );
    }

    #[test]
    fn test_generate_simple_model() {
        let json = json!({
            "name": "User",
            "fields": [
                {
                    "name": "id",
                    "type": "integer",
                    "primary_key": true
                },
                {
                    "name": "email",
                    "type": "string",
                    "unique": true,
                    "validations": [{"validation_type": "email"}]
                },
                {
                    "name": "age",
                    "type": "integer",
                    "nullable": true
                }
            ]
        });

        let result = generate_model_code(&json);
        assert!(result.is_ok(), "Generation should succeed");

        let code = result.unwrap();
        assert!(code.contains("pub struct Model"));
        assert!(code.contains("pub email: String"));
        assert!(code.contains("pub age: Option<i32>"));
        assert!(code.contains("#[validate(email)]"));
        assert!(code.contains("table_name = \"users\""));
    }

    #[test]
    fn test_generate_with_custom_table_name() {
        let json = json!({
            "name": "User",
            "table_name": "app_users",
            "fields": [
                {
                    "name": "id",
                    "type": "integer",
                    "primary_key": true
                },
                {
                    "name": "email",
                    "type": "string"
                }
            ]
        });

        let result = generate_model_code(&json);
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(code.contains("table_name = \"app_users\""));
    }

    #[test]
    fn test_generate_with_relationships() {
        let json = json!({
            "name": "Post",
            "fields": [
                {
                    "name": "id",
                    "type": "integer",
                    "primary_key": true
                },
                {
                    "name": "user_id",
                    "type": "integer"
                }
            ],
            "relationships": [
                {
                    "name": "user",
                    "model": "User",
                    "type": "belongs_to",
                    "foreign_key": "user_id"
                }
            ]
        });

        let result = generate_model_code(&json);
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(code.contains("belongs_to"));
        assert!(code.contains("super::user::Entity"));
    }

    #[test]
    fn test_generate_with_decimal() {
        let json = json!({
            "name": "Product",
            "fields": [
                {
                    "name": "id",
                    "type": "integer",
                    "primary_key": true
                },
                {
                    "name": "price",
                    "type": {
                        "decimal": {
                            "precision": 10,
                            "scale": 2
                        }
                    }
                }
            ]
        });

        let result = generate_model_code(&json);
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(code.contains("use rust_decimal::Decimal"));
        assert!(code.contains("pub price: Decimal"));
        assert!(code.contains("Decimal(Some((10, 2)))"));
    }

    #[test]
    fn test_feature_flag_detection() {
        let json = json!({
            "name": "Test",
            "fields": [
                {"name": "id", "type": "uuid", "primary_key": true},
                {"name": "created_at", "type": "timestamp"},
                {"name": "price", "type": {"decimal": {"precision": 10, "scale": 2}}},
                {"name": "email", "type": "string", "validations": [{"validation_type": "email"}]}
            ],
            "timestamps": false
        });

        let result = generate_model_code(&json);
        if result.is_err() {
            eprintln!("Error: {:?}", result.as_ref().unwrap_err());
        }
        assert!(result.is_ok());

        let code = result.unwrap();
        assert!(code.contains("use uuid::Uuid"));
        assert!(code.contains("use chrono::{DateTime, Utc}"));
        assert!(code.contains("use rust_decimal::Decimal"));
        assert!(code.contains("use validator::Validate"));
    }
}
