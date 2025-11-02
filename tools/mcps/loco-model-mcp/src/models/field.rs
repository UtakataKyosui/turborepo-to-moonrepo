// Field definition data structures

use serde::{Deserialize, Serialize};

/// Loco field types with corresponding Rust and SQL types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum LocoFieldType {
    /// String type (VARCHAR)
    String,
    /// Text type (TEXT) for longer content
    Text,
    /// Integer (i32)
    Integer,
    /// Small integer (i16)
    SmallInt,
    /// Big integer (i64)
    BigInt,
    /// UUID type
    Uuid,
    /// Boolean type
    Boolean,
    /// Timestamp (DateTime<Utc>)
    Timestamp,
    /// Date only
    Date,
    /// Time only
    Time,
    /// JSON type
    Json,
    /// JSONB (PostgreSQL binary JSON)
    Jsonb,
    /// Float (f32)
    Float,
    /// Double (f64)
    Double,
    /// Decimal with precision and scale
    Decimal { precision: u32, scale: u32 },
    /// Binary data (BLOB)
    Binary,
    /// Foreign key reference to another model
    Reference {
        model: String,
        column: Option<String>,
    },
}

impl LocoFieldType {
    /// Get the Rust type string for this field type
    pub fn to_rust_type(&self) -> String {
        match self {
            Self::String => "String".to_string(),
            Self::Text => "String".to_string(),
            Self::Integer => "i32".to_string(),
            Self::SmallInt => "i16".to_string(),
            Self::BigInt => "i64".to_string(),
            Self::Uuid => "Uuid".to_string(),
            Self::Boolean => "bool".to_string(),
            Self::Timestamp => "DateTime<Utc>".to_string(),
            Self::Date => "NaiveDate".to_string(),
            Self::Time => "NaiveTime".to_string(),
            Self::Json => "serde_json::Value".to_string(),
            Self::Jsonb => "serde_json::Value".to_string(),
            Self::Float => "f32".to_string(),
            Self::Double => "f64".to_string(),
            Self::Decimal { .. } => "Decimal".to_string(),
            Self::Binary => "Vec<u8>".to_string(),
            Self::Reference { model, .. } => format!("{}Id", model),
        }
    }

    /// Get the SeaORM column type for this field type
    pub fn to_sea_orm_type(&self) -> String {
        match self {
            Self::String => "String".to_string(),
            Self::Text => "Text".to_string(),
            Self::Integer => "Integer".to_string(),
            Self::SmallInt => "SmallInteger".to_string(),
            Self::BigInt => "BigInteger".to_string(),
            Self::Uuid => "Uuid".to_string(),
            Self::Boolean => "Boolean".to_string(),
            Self::Timestamp => "TimestampWithTimeZone".to_string(),
            Self::Date => "Date".to_string(),
            Self::Time => "Time".to_string(),
            Self::Json => "Json".to_string(),
            Self::Jsonb => "JsonBinary".to_string(),
            Self::Float => "Float".to_string(),
            Self::Double => "Double".to_string(),
            Self::Decimal { precision, scale } => {
                format!("Decimal(Some(({}, {})))", precision, scale)
            }
            Self::Binary => "Binary".to_string(),
            Self::Reference { .. } => "Integer".to_string(),
        }
    }
}

/// Default value for a field
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DefaultValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Null,
}

/// Field-level validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldValidation {
    pub validation_type: String,
    pub params: Option<serde_json::Value>,
}

/// Field definition for a model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    /// Field name in snake_case
    pub name: String,

    /// Field type
    #[serde(rename = "type")]
    pub field_type: LocoFieldType,

    /// Whether the field can be null
    #[serde(default)]
    pub nullable: bool,

    /// Whether the field must be unique
    #[serde(default)]
    pub unique: bool,

    /// Whether this is a primary key
    #[serde(default)]
    pub primary_key: bool,

    /// Default value for the field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<DefaultValue>,

    /// Whether to create an index on this field
    #[serde(default)]
    pub indexed: bool,

    /// Field-level validations
    #[serde(default)]
    pub validations: Vec<FieldValidation>,

    /// Optional description/comment
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl FieldDefinition {
    /// Create a new field definition
    pub fn new(name: String, field_type: LocoFieldType) -> Self {
        Self {
            name,
            field_type,
            nullable: false,
            unique: false,
            primary_key: false,
            default_value: None,
            indexed: false,
            validations: Vec::new(),
            description: None,
        }
    }

    /// Get the Loco field suffix based on nullable and unique flags
    /// string! = required, string^ = unique, string = nullable
    pub fn get_loco_suffix(&self) -> &str {
        if self.unique {
            "^"
        } else if !self.nullable {
            "!"
        } else {
            ""
        }
    }
}
