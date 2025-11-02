// Semantic validation - validates business logic and data consistency

use crate::error::{LocoModelError, Result};
use crate::models::field::{FieldDefinition, LocoFieldType};
use crate::models::specification::ModelSpecification;
use crate::models::validation::{ValidationRule, ValidationRuleType};
use std::collections::{HashMap, HashSet};

/// Validate model specification semantics
pub fn validate_model_semantics(spec: &ModelSpecification) -> Result<()> {
    // Validate field name uniqueness
    validate_field_name_uniqueness(spec)?;

    // Validate timestamp field conflicts
    validate_timestamp_conflicts(spec)?;

    // Validate primary key configuration
    validate_primary_keys(spec)?;

    // Validate field types and constraints
    validate_field_types(spec)?;

    // Validate relationships
    validate_relationships(spec)?;

    // Validate validation rules compatibility
    validate_validation_rules(spec)?;

    Ok(())
}

/// Validate that field names are unique within the model
fn validate_field_name_uniqueness(spec: &ModelSpecification) -> Result<()> {
    let mut seen_names: HashSet<String> = HashSet::new();

    for field in &spec.fields {
        if seen_names.contains(&field.name) {
            return Err(LocoModelError::Validation(format!(
                "Duplicate field name '{}' in model '{}'",
                field.name, spec.name
            )));
        }
        seen_names.insert(field.name.clone());
    }

    Ok(())
}

/// Validate that field names don't conflict with timestamp fields
fn validate_timestamp_conflicts(spec: &ModelSpecification) -> Result<()> {
    if spec.timestamps {
        let reserved_names = vec!["created_at", "updated_at"];

        for field in &spec.fields {
            if reserved_names.contains(&field.name.as_str()) {
                return Err(LocoModelError::Validation(format!(
                    "Field name '{}' conflicts with automatic timestamp field. \
                    Either rename the field or set 'timestamps: false'",
                    field.name
                )));
            }
        }
    }

    Ok(())
}

/// Validate primary key configuration
fn validate_primary_keys(spec: &ModelSpecification) -> Result<()> {
    let primary_keys: Vec<&FieldDefinition> = spec.fields
        .iter()
        .filter(|f| f.primary_key)
        .collect();

    // Check if there's at least one primary key
    if primary_keys.is_empty() {
        return Err(LocoModelError::Validation(format!(
            "Model '{}' must have at least one primary key field",
            spec.name
        )));
    }

    // SeaORM supports composite primary keys, but validate type compatibility
    for pk in &primary_keys {
        validate_primary_key_type(pk)?;
    }

    // Ensure there are non-primary-key fields (models need data)
    let data_fields: Vec<&FieldDefinition> = spec.fields
        .iter()
        .filter(|f| !f.primary_key)
        .collect();

    if data_fields.is_empty() {
        return Err(LocoModelError::Validation(format!(
            "Model '{}' must have at least one non-primary-key field",
            spec.name
        )));
    }

    Ok(())
}

/// Validate that primary key has appropriate type
fn validate_primary_key_type(field: &FieldDefinition) -> Result<()> {
    let valid_pk_types = vec![
        "integer", "big_int", "small_int", "uuid", "string"
    ];

    let type_str = match &field.field_type {
        LocoFieldType::String => "string",
        LocoFieldType::Text => "text",
        LocoFieldType::Integer => "integer",
        LocoFieldType::SmallInt => "small_int",
        LocoFieldType::BigInt => "big_int",
        LocoFieldType::Uuid => "uuid",
        LocoFieldType::Boolean => "boolean",
        LocoFieldType::Timestamp => "timestamp",
        LocoFieldType::Date => "date",
        LocoFieldType::Time => "time",
        LocoFieldType::Json => "json",
        LocoFieldType::Jsonb => "jsonb",
        LocoFieldType::Float => "float",
        LocoFieldType::Double => "double",
        LocoFieldType::Binary => "binary",
        LocoFieldType::Decimal { .. } => "decimal",
        LocoFieldType::Reference { .. } => "reference",
    };

    if !valid_pk_types.contains(&type_str) {
        return Err(LocoModelError::Validation(format!(
            "Primary key field '{}' has invalid type '{}'. \
            Valid primary key types are: {}",
            field.name,
            type_str,
            valid_pk_types.join(", ")
        )));
    }

    Ok(())
}

/// Validate field types and their constraints
fn validate_field_types(spec: &ModelSpecification) -> Result<()> {
    for field in &spec.fields {
        // Validate decimal precision and scale
        if let LocoFieldType::Decimal { precision, scale } = &field.field_type {
            if *precision == 0 {
                return Err(LocoModelError::InvalidFieldType(format!(
                    "Decimal field '{}' must have precision > 0",
                    field.name
                )));
            }

            if *scale > *precision {
                return Err(LocoModelError::InvalidFieldType(format!(
                    "Decimal field '{}' has scale ({}) greater than precision ({})",
                    field.name, scale, precision
                )));
            }

            // PostgreSQL limit
            if *precision > 1000 {
                return Err(LocoModelError::InvalidFieldType(format!(
                    "Decimal field '{}' has precision ({}) exceeding PostgreSQL limit (1000)",
                    field.name, precision
                )));
            }
        }

        // Validate that nullable primary keys are warned about (SeaORM allows but uncommon)
        if field.primary_key && field.nullable {
            // This is more of a warning, but we'll allow it as SeaORM supports it
            // In production, you might want to log a warning here
        }
    }

    Ok(())
}

/// Validate relationships
fn validate_relationships(spec: &ModelSpecification) -> Result<()> {
    use crate::models::relationship::RelationshipDefinition as RD;

    // Track relationship names for uniqueness
    let mut relationship_names: HashSet<String> = HashSet::new();

    for relationship in &spec.relationships {
        let (name, foreign_key, is_belongs_to) = match relationship {
            RD::BelongsTo { name, foreign_key, .. } => (name, foreign_key, true),
            RD::HasMany { name, foreign_key, .. } => (name, foreign_key, false),
            RD::HasOne { name, foreign_key, .. } => (name, foreign_key, false),
            RD::ManyToMany { name, .. } => (name, &None, false),
        };

        // Check for duplicate relationship names
        if relationship_names.contains(name) {
            return Err(LocoModelError::Validation(format!(
                "Duplicate relationship name '{}' in model '{}'",
                name, spec.name
            )));
        }
        relationship_names.insert(name.clone());

        // Validate belongs_to has foreign key
        if is_belongs_to && foreign_key.is_none() {
            return Err(LocoModelError::Validation(format!(
                "Relationship '{}' of type 'belongs_to' must specify a foreign_key",
                name
            )));
        }

        // Validate that foreign key field exists (if specified)
        if let Some(fk) = foreign_key {
            let fk_exists = spec.fields.iter().any(|f| &f.name == fk);
            if !fk_exists {
                return Err(LocoModelError::Validation(format!(
                    "Foreign key field '{}' specified in relationship '{}' does not exist in model '{}'",
                    fk, name, spec.name
                )));
            }
        }
    }

    Ok(())
}

/// Validate that validation rules are compatible with field types
fn validate_validation_rules(spec: &ModelSpecification) -> Result<()> {
    // Build a map of field names to their types for quick lookup
    let field_types: HashMap<String, &LocoFieldType> = spec.fields
        .iter()
        .map(|f| (f.name.clone(), &f.field_type))
        .collect();

    for validation in &spec.validations {
        // Check that the field exists
        if !field_types.contains_key(&validation.field) {
            return Err(LocoModelError::Validation(format!(
                "Validation rule references non-existent field '{}'",
                validation.field
            )));
        }

        let field_type = field_types.get(&validation.field).unwrap();

        // Validate rule compatibility with field type
        match &validation.rule {
            ValidationRuleType::Email => {
                    // Email validation only makes sense for string/text
                    if !matches!(field_type,
                        LocoFieldType::String |
                        LocoFieldType::Text) {
                        return Err(LocoModelError::Validation(format!(
                            "Email validation rule on field '{}' requires string or text type",
                            validation.field
                        )));
                    }
                }
                ValidationRuleType::Url => {
                    // URL validation only makes sense for string/text
                    if !matches!(field_type,
                        LocoFieldType::String |
                        LocoFieldType::Text) {
                        return Err(LocoModelError::Validation(format!(
                            "URL validation rule on field '{}' requires string or text type",
                            validation.field
                        )));
                    }
                }
                ValidationRuleType::Length { min, max } => {
                    // Length validation for string/text
                    if !matches!(field_type,
                        LocoFieldType::String |
                        LocoFieldType::Text) {
                        return Err(LocoModelError::Validation(format!(
                            "Length validation rule on field '{}' requires string or text type",
                            validation.field
                        )));
                    }

                    // Validate min <= max
                    if let (Some(min_val), Some(max_val)) = (min, max) {
                        if min_val > max_val {
                            return Err(LocoModelError::Validation(format!(
                                "Length validation on field '{}' has min ({}) > max ({})",
                                validation.field, min_val, max_val
                            )));
                        }
                    }
                }
                ValidationRuleType::Range { min, max } => {
                    // Range validation for numeric types
                    if !matches!(field_type,
                        LocoFieldType::Integer |
                        LocoFieldType::SmallInt |
                        LocoFieldType::BigInt |
                        LocoFieldType::Float |
                        LocoFieldType::Double |
                        LocoFieldType::Decimal { .. }) {
                        return Err(LocoModelError::Validation(format!(
                            "Range validation rule on field '{}' requires numeric type",
                            validation.field
                        )));
                    }

                    // Validate min <= max
                    if let (Some(min_val), Some(max_val)) = (min, max) {
                        if min_val > max_val {
                            return Err(LocoModelError::Validation(format!(
                                "Range validation on field '{}' has min ({}) > max ({})",
                                validation.field, min_val, max_val
                            )));
                        }
                    }
                }
                ValidationRuleType::Regex { .. } => {
                    // Regex validation for string/text
                    if !matches!(field_type,
                        LocoFieldType::String |
                        LocoFieldType::Text) {
                        return Err(LocoModelError::Validation(format!(
                            "Regex validation rule on field '{}' requires string or text type",
                            validation.field
                        )));
                    }
                }
                ValidationRuleType::Custom { .. } => {
                    // Custom validation can apply to any type
                }
                ValidationRuleType::Required => {
                    // Required validation applies to all types
                }
                ValidationRuleType::In { .. } => {
                    // In validation for string/enum types
                    if !matches!(field_type,
                        LocoFieldType::String |
                        LocoFieldType::Text) {
                        return Err(LocoModelError::Validation(format!(
                            "In validation rule on field '{}' is typically used with string or text types",
                            validation.field
                        )));
                    }
                }
                ValidationRuleType::NotIn { .. } => {
                    // NotIn validation for string/enum types
                    if !matches!(field_type,
                        LocoFieldType::String |
                        LocoFieldType::Text) {
                        return Err(LocoModelError::Validation(format!(
                            "NotIn validation rule on field '{}' is typically used with string or text types",
                            validation.field
                        )));
                    }
                }
            }
        }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_model_semantics() {
        let spec = ModelSpecification {
            name: "User".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "email".to_string(),
                    field_type: LocoFieldType::String,
                    nullable: false,
                    unique: true,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: true,
            relationships: vec![],
            validations: vec![],
            hooks: vec![],
            
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_ok());
    }

    #[test]
    fn test_duplicate_field_names() {
        let spec = ModelSpecification {
            name: "User".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "email".to_string(),
                    field_type: LocoFieldType::String,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "email".to_string(),
                    field_type: LocoFieldType::String,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![],
            validations: vec![],
            hooks: vec![],
            
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Duplicate field name"));
    }

    #[test]
    fn test_timestamp_field_conflict() {
        let spec = ModelSpecification {
            name: "User".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "created_at".to_string(),
                    field_type: LocoFieldType::Timestamp,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: true,
            relationships: vec![],
            validations: vec![],
            hooks: vec![],
            
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("conflicts with automatic timestamp"));
    }

    #[test]
    fn test_missing_primary_key() {
        let spec = ModelSpecification {
            name: "User".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "email".to_string(),
                    field_type: LocoFieldType::String,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![],
            validations: vec![],
            hooks: vec![],
            
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must have at least one primary key"));
    }

    #[test]
    fn test_only_primary_keys() {
        let spec = ModelSpecification {
            name: "User".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![],
            validations: vec![],
            hooks: vec![],
            
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must have at least one non-primary-key field"));
    }

    #[test]
    fn test_invalid_primary_key_type() {
        let spec = ModelSpecification {
            name: "User".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Boolean,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "name".to_string(),
                    field_type: LocoFieldType::String,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![],
            validations: vec![],
            hooks: vec![],
            
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("has invalid type"));
    }

    #[test]
    fn test_invalid_decimal_precision() {
        let spec = ModelSpecification {
            name: "Product".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "price".to_string(),
                    field_type: LocoFieldType::Decimal {
                        precision: 0,
                        scale: 2,
                    },
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![],
            validations: vec![],
            hooks: vec![],
            
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("precision > 0"));
    }

    #[test]
    fn test_decimal_scale_exceeds_precision() {
        let spec = ModelSpecification {
            name: "Product".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "price".to_string(),
                    field_type: LocoFieldType::Decimal {
                        precision: 5,
                        scale: 10,
                    },
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![],
            validations: vec![],
            hooks: vec![],
            
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("scale") && err_msg.contains("precision"));
    }

    #[test]
    fn test_belongs_to_missing_foreign_key() {
        let spec = ModelSpecification {
            name: "Post".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "title".to_string(),
                    field_type: LocoFieldType::String,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![
                crate::models::relationship::RelationshipDefinition::BelongsTo {
                    name: "user".to_string(),
                    model: "User".to_string(),
                    foreign_key: None,
                    condition: None,
                },
            ],
            validations: vec![],
            hooks: vec![],
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("must specify a foreign_key"));
    }

    #[test]
    fn test_foreign_key_field_not_exists() {
        let spec = ModelSpecification {
            name: "Post".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "title".to_string(),
                    field_type: LocoFieldType::String,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![
                crate::models::relationship::RelationshipDefinition::BelongsTo {
                    name: "user".to_string(),
                    model: "User".to_string(),
                    foreign_key: Some("user_id".to_string()),
                    condition: None,
                },
            ],
            validations: vec![],
            hooks: vec![],
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("does not exist"));
    }

    #[test]
    fn test_validation_rule_non_existent_field() {
        let spec = ModelSpecification {
            name: "User".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "name".to_string(),
                    field_type: LocoFieldType::String,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![],
            validations: vec![
                ValidationRule {
                    field: "email".to_string(),
                    rule: ValidationRuleType::Email,
                },
            ],
            hooks: vec![],
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("non-existent field"));
    }

    #[test]
    fn test_email_validation_on_non_string() {
        let spec = ModelSpecification {
            name: "User".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "age".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![],
            validations: vec![
                ValidationRule {
                    field: "age".to_string(),
                    rule: ValidationRuleType::Email,
                },
            ],
            hooks: vec![],
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("requires string or text type"));
    }

    #[test]
    fn test_length_validation_min_greater_than_max() {
        let spec = ModelSpecification {
            name: "User".to_string(),
            table_name: None,
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: LocoFieldType::Integer,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: true,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
                FieldDefinition {
                    name: "name".to_string(),
                    field_type: LocoFieldType::String,
                    nullable: false,
                    unique: false,
                    indexed: false,
                    primary_key: false,
                    default_value: None,
                    description: None,
                    validations: vec![],
                    
                },
            ],
            timestamps: false,
            relationships: vec![],
            validations: vec![
                ValidationRule {
                    field: "name".to_string(),
                    rule: ValidationRuleType::Length {
                        min: Some(10),
                        max: Some(5),
                    },
                },
            ],
            hooks: vec![],
            generate_migration: true,
        };

        let result = validate_model_semantics(&spec);
        assert!(result.is_err());
        let err_msg = result.unwrap_err().to_string();
        assert!(err_msg.contains("min") && err_msg.contains("max"));
    }
}
