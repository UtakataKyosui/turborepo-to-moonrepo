// Model specification data structure

use serde::{Deserialize, Serialize};

use super::field::FieldDefinition;
use super::hook::LifecycleHook;
use super::relationship::RelationshipDefinition;
use super::validation::ValidationRule;

/// Complete specification for a Loco model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelSpecification {
    /// Model name in PascalCase (e.g., "User", "BlogPost")
    pub name: String,

    /// Optional custom table name (defaults to snake_case plural)
    pub table_name: Option<String>,

    /// Field definitions for the model
    pub fields: Vec<FieldDefinition>,

    /// Relationship definitions (belongs_to, has_many, etc.)
    #[serde(default)]
    pub relationships: Vec<RelationshipDefinition>,

    /// Validation rules for fields
    #[serde(default)]
    pub validations: Vec<ValidationRule>,

    /// Lifecycle hooks (before_save, after_create, etc.)
    #[serde(default)]
    pub hooks: Vec<LifecycleHook>,

    /// Whether to include created_at/updated_at timestamps
    #[serde(default = "default_true")]
    pub timestamps: bool,

    /// Whether to generate a migration file
    #[serde(default = "default_true")]
    pub generate_migration: bool,
}

fn default_true() -> bool {
    true
}

impl ModelSpecification {
    /// Create a new model specification with the given name and fields
    pub fn new(name: String, fields: Vec<FieldDefinition>) -> Self {
        Self {
            name,
            table_name: None,
            fields,
            relationships: Vec::new(),
            validations: Vec::new(),
            hooks: Vec::new(),
            timestamps: true,
            generate_migration: true,
        }
    }

    /// Validate the model specification
    pub fn validate(&self) -> crate::error::Result<()> {
        // Validate model name
        if self.name.is_empty() {
            return Err(crate::error::LocoModelError::InvalidModelName(
                "Model name cannot be empty".to_string(),
            ));
        }

        // Validate fields
        if self.fields.is_empty() {
            return Err(crate::error::LocoModelError::Validation(
                "Model must have at least one field".to_string(),
            ));
        }

        // Check for duplicate field names
        let mut field_names = std::collections::HashSet::new();
        for field in &self.fields {
            if !field_names.insert(&field.name) {
                return Err(crate::error::LocoModelError::DuplicateField(
                    field.name.clone(),
                ));
            }
        }

        Ok(())
    }
}
