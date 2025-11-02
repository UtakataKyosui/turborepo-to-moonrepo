// Parse validation - validates input JSON format and structure

use crate::error::{LocoModelError, Result};
use crate::models::specification::ModelSpecification;
use serde_json::Value;

/// Validate and parse model specification from JSON
pub fn parse_model_spec(json: &Value) -> Result<ModelSpecification> {
    // Validate that it's an object
    if !json.is_object() {
        return Err(LocoModelError::InvalidToolParams(
            "Model specification must be a JSON object".to_string(),
        ));
    }

    // Validate required fields exist
    validate_required_fields(json)?;

    // Attempt to deserialize
    let spec: ModelSpecification = serde_json::from_value(json.clone())
        .map_err(|e| LocoModelError::InvalidToolParams(format!("Failed to parse model specification: {}", e)))?;

    Ok(spec)
}

/// Validate that all required fields are present in the JSON
fn validate_required_fields(json: &Value) -> Result<()> {
    let obj = json.as_object().ok_or_else(|| {
        LocoModelError::InvalidToolParams("Expected JSON object".to_string())
    })?;

    // Check for required 'name' field
    if !obj.contains_key("name") {
        return Err(LocoModelError::InvalidToolParams(
            "Missing required field: 'name'".to_string(),
        ));
    }

    // Validate name is a string
    if let Some(name) = obj.get("name") {
        if !name.is_string() {
            return Err(LocoModelError::InvalidToolParams(
                "'name' field must be a string".to_string(),
            ));
        }
        if name.as_str().unwrap().is_empty() {
            return Err(LocoModelError::InvalidModelName(
                "Model name cannot be empty".to_string(),
            ));
        }
    }

    // Check for required 'fields' array
    if !obj.contains_key("fields") {
        return Err(LocoModelError::InvalidToolParams(
            "Missing required field: 'fields'".to_string(),
        ));
    }

    // Validate fields is an array
    if let Some(fields) = obj.get("fields") {
        if !fields.is_array() {
            return Err(LocoModelError::InvalidToolParams(
                "'fields' must be an array".to_string(),
            ));
        }

        let fields_array = fields.as_array().unwrap();
        if fields_array.is_empty() {
            return Err(LocoModelError::Validation(
                "Model must have at least one field".to_string(),
            ));
        }

        // Validate each field object
        for (idx, field) in fields_array.iter().enumerate() {
            validate_field_object(field, idx)?;
        }
    }

    // Validate optional arrays if present
    if let Some(relationships) = obj.get("relationships") {
        if !relationships.is_null() && !relationships.is_array() {
            return Err(LocoModelError::InvalidToolParams(
                "'relationships' must be an array".to_string(),
            ));
        }
    }

    if let Some(validations) = obj.get("validations") {
        if !validations.is_null() && !validations.is_array() {
            return Err(LocoModelError::InvalidToolParams(
                "'validations' must be an array".to_string(),
            ));
        }
    }

    if let Some(hooks) = obj.get("hooks") {
        if !hooks.is_null() && !hooks.is_array() {
            return Err(LocoModelError::InvalidToolParams(
                "'hooks' must be an array".to_string(),
            ));
        }
    }

    Ok(())
}

/// Validate a field object structure
fn validate_field_object(field: &Value, index: usize) -> Result<()> {
    let field_obj = field.as_object().ok_or_else(|| {
        LocoModelError::InvalidToolParams(format!(
            "Field at index {} must be a JSON object",
            index
        ))
    })?;

    // Check for required 'name' field
    if !field_obj.contains_key("name") {
        return Err(LocoModelError::InvalidToolParams(format!(
            "Field at index {} is missing required 'name' field",
            index
        )));
    }

    if let Some(name) = field_obj.get("name") {
        if !name.is_string() {
            return Err(LocoModelError::InvalidFieldName(format!(
                "Field name at index {} must be a string",
                index
            )));
        }
        validate_field_name(name.as_str().unwrap(), index)?;
    }

    // Check for required 'type' field
    if !field_obj.contains_key("type") {
        return Err(LocoModelError::InvalidToolParams(format!(
            "Field '{}' at index {} is missing required 'type' field",
            field_obj.get("name").and_then(|n| n.as_str()).unwrap_or("unknown"),
            index
        )));
    }

    // Validate type format
    if let Some(field_type) = field_obj.get("type") {
        validate_field_type_format(field_type, index)?;
    }

    Ok(())
}

/// Validate field name format
fn validate_field_name(name: &str, index: usize) -> Result<()> {
    if name.is_empty() {
        return Err(LocoModelError::InvalidFieldName(format!(
            "Field name at index {} cannot be empty",
            index
        )));
    }

    // Check for valid identifier (snake_case convention)
    if !name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_') {
        return Err(LocoModelError::InvalidFieldName(format!(
            "Field name '{}' at index {} must be snake_case (lowercase, digits, underscores only)",
            name, index
        )));
    }

    // Must not start with a digit
    if name.chars().next().unwrap().is_ascii_digit() {
        return Err(LocoModelError::InvalidFieldName(format!(
            "Field name '{}' at index {} cannot start with a digit",
            name, index
        )));
    }

    // Check for Rust reserved keywords
    const RESERVED_KEYWORDS: &[&str] = &[
        "as", "break", "const", "continue", "crate", "else", "enum", "extern",
        "false", "fn", "for", "if", "impl", "in", "let", "loop", "match",
        "mod", "move", "mut", "pub", "ref", "return", "self", "Self", "static",
        "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
        "async", "await", "dyn", "abstract", "become", "box", "do", "final",
        "macro", "override", "priv", "typeof", "unsized", "virtual", "yield", "try",
    ];

    if RESERVED_KEYWORDS.contains(&name) {
        return Err(LocoModelError::InvalidFieldName(format!(
            "Field name '{}' at index {} is a Rust reserved keyword",
            name, index
        )));
    }

    Ok(())
}

/// Validate field type format
fn validate_field_type_format(field_type: &Value, index: usize) -> Result<()> {
    // Type can be a string (simple type) or object (complex type like decimal)
    if !field_type.is_string() && !field_type.is_object() {
        return Err(LocoModelError::InvalidFieldType(format!(
            "Field type at index {} must be a string or object",
            index
        )));
    }

    // If it's a string, validate it's a known type
    if let Some(type_str) = field_type.as_str() {
        const VALID_TYPES: &[&str] = &[
            "string", "text", "integer", "small_int", "big_int", "uuid",
            "boolean", "timestamp", "date", "time", "json", "jsonb",
            "float", "double", "binary",
        ];

        if !VALID_TYPES.contains(&type_str) {
            return Err(LocoModelError::InvalidFieldType(format!(
                "Unknown field type '{}' at index {}. Valid types: {}",
                type_str,
                index,
                VALID_TYPES.join(", ")
            )));
        }
    }

    // If it's an object, validate it has required structure
    if let Some(type_obj) = field_type.as_object() {
        // For decimal type
        if type_obj.contains_key("decimal") {
            if let Some(decimal_config) = type_obj.get("decimal") {
                let decimal_obj = decimal_config.as_object().ok_or_else(|| {
                    LocoModelError::InvalidFieldType(format!(
                        "Decimal configuration at index {} must be an object",
                        index
                    ))
                })?;

                // Validate precision and scale
                if !decimal_obj.contains_key("precision") || !decimal_obj.contains_key("scale") {
                    return Err(LocoModelError::InvalidFieldType(format!(
                        "Decimal type at index {} must have 'precision' and 'scale' fields",
                        index
                    )));
                }

                if !decimal_obj["precision"].is_number() || !decimal_obj["scale"].is_number() {
                    return Err(LocoModelError::InvalidFieldType(format!(
                        "Decimal precision and scale at index {} must be numbers",
                        index
                    )));
                }
            }
        }
        // For reference type
        else if type_obj.contains_key("reference") {
            if let Some(ref_config) = type_obj.get("reference") {
                let ref_obj = ref_config.as_object().ok_or_else(|| {
                    LocoModelError::InvalidFieldType(format!(
                        "Reference configuration at index {} must be an object",
                        index
                    ))
                })?;

                if !ref_obj.contains_key("model") {
                    return Err(LocoModelError::InvalidFieldType(format!(
                        "Reference type at index {} must have 'model' field",
                        index
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
    use serde_json::json;

    #[test]
    fn test_parse_valid_model() {
        let json = json!({
            "name": "User",
            "fields": [
                {"name": "email", "type": "string"},
                {"name": "age", "type": "integer"}
            ]
        });

        let result = parse_model_spec(&json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_missing_name() {
        let json = json!({
            "fields": [
                {"name": "email", "type": "string"}
            ]
        });

        let result = parse_model_spec(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("name"));
    }

    #[test]
    fn test_parse_empty_name() {
        let json = json!({
            "name": "",
            "fields": [
                {"name": "email", "type": "string"}
            ]
        });

        let result = parse_model_spec(&json);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_missing_fields() {
        let json = json!({
            "name": "User"
        });

        let result = parse_model_spec(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("fields"));
    }

    #[test]
    fn test_parse_empty_fields() {
        let json = json!({
            "name": "User",
            "fields": []
        });

        let result = parse_model_spec(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("at least one field"));
    }

    #[test]
    fn test_validate_field_name_invalid_chars() {
        let json = json!({
            "name": "User",
            "fields": [
                {"name": "Email-Address", "type": "string"}
            ]
        });

        let result = parse_model_spec(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("snake_case"));
    }

    #[test]
    fn test_validate_field_name_reserved_keyword() {
        let json = json!({
            "name": "User",
            "fields": [
                {"name": "type", "type": "string"}
            ]
        });

        let result = parse_model_spec(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("reserved keyword"));
    }

    #[test]
    fn test_validate_unknown_field_type() {
        let json = json!({
            "name": "User",
            "fields": [
                {"name": "email", "type": "unknown_type"}
            ]
        });

        let result = parse_model_spec(&json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Unknown field type"));
    }

    #[test]
    fn test_validate_decimal_type() {
        let json = json!({
            "name": "Product",
            "fields": [
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

        let result = parse_model_spec(&json);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_reference_type() {
        let json = json!({
            "name": "Post",
            "fields": [
                {
                    "name": "user_id",
                    "type": {
                        "reference": {
                            "model": "User"
                        }
                    }
                }
            ]
        });

        let result = parse_model_spec(&json);
        assert!(result.is_ok());
    }
}
