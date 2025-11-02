// Template manager for Tera templates

use crate::error::{LocoModelError, Result};
use once_cell::sync::Lazy;
use serde::Serialize;
use std::collections::HashMap;
use tera::Tera;

/// Global Tera template instance
static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    // Initialize Tera with templates directory
    match Tera::new("templates/**/*.tera") {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Failed to parse templates: {}", e);
            // Return empty Tera instance as fallback
            Tera::default()
        }
    }
});

/// Template context for model generation
#[derive(Debug, Clone, Serialize)]
pub struct ModelTemplateContext {
    /// Model name (PascalCase)
    pub name: String,

    /// Table name (snake_case)
    pub table_name: String,

    /// Model description (optional)
    pub description: Option<String>,

    /// Fields with their metadata
    pub fields: Vec<FieldContext>,

    /// Relationships (optional)
    pub relationships: Vec<RelationshipContext>,

    /// Lifecycle hooks (optional)
    pub hooks: Vec<HookContext>,

    /// Feature flags
    pub has_uuid: bool,
    pub has_datetime: bool,
    pub has_date: bool,
    pub has_time: bool,
    pub has_decimal: bool,
    pub has_validations: bool,
}

/// Field context for template rendering
#[derive(Debug, Clone, Serialize)]
pub struct FieldContext {
    /// Field name (snake_case)
    pub name: String,

    /// Rust type string
    pub rust_type: String,

    /// SeaORM column_type override (optional)
    pub column_type: Option<String>,

    /// Field description (optional)
    pub description: Option<String>,

    /// Field flags
    pub primary_key: bool,
    pub unique: bool,
    pub indexed: bool,

    /// Validation attributes (for validator crate)
    pub validations: Vec<String>,
}

/// Relationship context for template rendering
#[derive(Debug, Clone, Serialize)]
pub struct RelationshipContext {
    /// Relationship name (PascalCase)
    pub name_pascal: String,

    /// Related model name (snake_case)
    pub model_snake: String,

    /// Relationship type (belongs_to, has_many, has_one)
    #[serde(rename = "type")]
    pub relationship_type: String,

    /// Foreign key field (PascalCase) for belongs_to
    pub foreign_key_pascal: Option<String>,
}

/// Hook context for template rendering
#[derive(Debug, Clone, Serialize)]
pub struct HookContext {
    /// Hook type (before_save, after_save, etc.)
    #[serde(rename = "type")]
    pub hook_type: String,

    /// Custom hook code (optional)
    pub code: Option<String>,
}

/// Render the model template with the given context
pub fn render_model(context: &ModelTemplateContext) -> Result<String> {
    // Convert context to Tera context
    let mut tera_context = tera::Context::new();
    tera_context.insert("name", &context.name);
    tera_context.insert("table_name", &context.table_name);
    tera_context.insert("description", &context.description);
    tera_context.insert("fields", &context.fields);
    tera_context.insert("relationships", &context.relationships);
    tera_context.insert("hooks", &context.hooks);
    tera_context.insert("has_uuid", &context.has_uuid);
    tera_context.insert("has_datetime", &context.has_datetime);
    tera_context.insert("has_date", &context.has_date);
    tera_context.insert("has_time", &context.has_time);
    tera_context.insert("has_decimal", &context.has_decimal);
    tera_context.insert("has_validations", &context.has_validations);

    // Render the template
    TEMPLATES
        .render("model.tera", &tera_context)
        .map_err(|e| {
            LocoModelError::Generation(format!("Failed to render template: {}", e))
        })
}

/// Get available template names
pub fn get_template_names() -> Vec<&'static str> {
    TEMPLATES.get_template_names().collect()
}

/// Check if a template exists
pub fn template_exists(name: &str) -> bool {
    TEMPLATES.get_template_names().any(|t| t == name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_loading() {
        // Check that model.tera template is loaded
        let names = get_template_names();
        assert!(
            names.iter().any(|&n| n == "model.tera"),
            "model.tera template should be loaded. Available templates: {:?}",
            names
        );
    }

    #[test]
    fn test_template_exists() {
        assert!(template_exists("model.tera"), "model.tera should exist");
    }

    #[test]
    fn test_render_simple_model() {
        let context = ModelTemplateContext {
            name: "User".to_string(),
            table_name: "users".to_string(),
            description: Some("User model".to_string()),
            fields: vec![
                FieldContext {
                    name: "id".to_string(),
                    rust_type: "i32".to_string(),
                    column_type: None,
                    description: None,
                    primary_key: true,
                    unique: false,
                    indexed: false,
                    validations: vec![],
                },
                FieldContext {
                    name: "email".to_string(),
                    rust_type: "String".to_string(),
                    column_type: None,
                    description: Some("User email address".to_string()),
                    primary_key: false,
                    unique: true,
                    indexed: false,
                    validations: vec!["#[validate(email)]".to_string()],
                },
            ],
            relationships: vec![],
            hooks: vec![],
            has_uuid: false,
            has_datetime: false,
            has_date: false,
            has_time: false,
            has_decimal: false,
            has_validations: true,
        };

        let result = render_model(&context);
        assert!(result.is_ok(), "Rendering should succeed");

        let code = result.unwrap();
        assert!(code.contains("pub struct Model"));
        assert!(code.contains("users"));
        assert!(code.contains("email"));
        assert!(code.contains("Validate"));
    }

    #[test]
    fn test_render_with_relationships() {
        let context = ModelTemplateContext {
            name: "Post".to_string(),
            table_name: "posts".to_string(),
            description: None,
            fields: vec![
                FieldContext {
                    name: "id".to_string(),
                    rust_type: "i32".to_string(),
                    column_type: None,
                    description: None,
                    primary_key: true,
                    unique: false,
                    indexed: false,
                    validations: vec![],
                },
                FieldContext {
                    name: "user_id".to_string(),
                    rust_type: "i32".to_string(),
                    column_type: None,
                    description: None,
                    primary_key: false,
                    unique: false,
                    indexed: true,
                    validations: vec![],
                },
            ],
            relationships: vec![RelationshipContext {
                name_pascal: "User".to_string(),
                model_snake: "user".to_string(),
                relationship_type: "belongs_to".to_string(),
                foreign_key_pascal: Some("UserId".to_string()),
            }],
            hooks: vec![],
            has_uuid: false,
            has_datetime: false,
            has_date: false,
            has_time: false,
            has_decimal: false,
            has_validations: false,
        };

        let result = render_model(&context);
        assert!(result.is_ok(), "Rendering should succeed");

        let code = result.unwrap();
        assert!(code.contains("belongs_to"));
        assert!(code.contains("user"));
        assert!(code.contains("UserId"));
    }

    #[test]
    fn test_render_with_uuid() {
        let context = ModelTemplateContext {
            name: "Account".to_string(),
            table_name: "accounts".to_string(),
            description: None,
            fields: vec![FieldContext {
                name: "id".to_string(),
                rust_type: "Uuid".to_string(),
                column_type: None,
                description: None,
                primary_key: true,
                unique: false,
                indexed: false,
                validations: vec![],
            }],
            relationships: vec![],
            hooks: vec![],
            has_uuid: true,
            has_datetime: false,
            has_date: false,
            has_time: false,
            has_decimal: false,
            has_validations: false,
        };

        let result = render_model(&context);
        assert!(result.is_ok(), "Rendering should succeed");

        let code = result.unwrap();
        assert!(code.contains("use uuid::Uuid"));
    }

    #[test]
    fn test_render_with_decimal() {
        let context = ModelTemplateContext {
            name: "Product".to_string(),
            table_name: "products".to_string(),
            description: None,
            fields: vec![
                FieldContext {
                    name: "id".to_string(),
                    rust_type: "i32".to_string(),
                    column_type: None,
                    description: None,
                    primary_key: true,
                    unique: false,
                    indexed: false,
                    validations: vec![],
                },
                FieldContext {
                    name: "price".to_string(),
                    rust_type: "Decimal".to_string(),
                    column_type: Some("Decimal(Some((10, 2)))".to_string()),
                    description: None,
                    primary_key: false,
                    unique: false,
                    indexed: false,
                    validations: vec![],
                },
            ],
            relationships: vec![],
            hooks: vec![],
            has_uuid: false,
            has_datetime: false,
            has_date: false,
            has_time: false,
            has_decimal: true,
            has_validations: false,
        };

        let result = render_model(&context);
        assert!(result.is_ok(), "Rendering should succeed");

        let code = result.unwrap();
        assert!(code.contains("use rust_decimal::Decimal"));
        assert!(code.contains("Decimal(Some((10, 2)))"));
    }
}
