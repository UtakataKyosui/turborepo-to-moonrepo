// Generated model output data structures

use serde::{Deserialize, Serialize};

/// Generated model code output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedModel {
    /// Model name
    pub model_name: String,

    /// Generated Rust code for the model
    pub code: String,

    /// File path where the model should be saved
    pub file_path: String,

    /// Optional migration code if requested
    #[serde(skip_serializing_if = "Option::is_none")]
    pub migration: Option<GeneratedMigration>,

    /// Metadata about the generation
    pub metadata: GenerationMetadata,
}

/// Generated migration code
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneratedMigration {
    /// Migration name/timestamp
    pub name: String,

    /// Migration up code
    pub up_code: String,

    /// Migration down code
    pub down_code: String,

    /// File path where the migration should be saved
    pub file_path: String,
}

/// Metadata about the code generation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationMetadata {
    /// Timestamp when generated
    #[serde(with = "chrono::serde::ts_seconds")]
    pub generated_at: chrono::DateTime<chrono::Utc>,

    /// Generator version
    pub generator_version: String,

    /// Number of fields in the model
    pub field_count: usize,

    /// Number of relationships
    pub relationship_count: usize,

    /// Number of validations
    pub validation_count: usize,

    /// Number of hooks
    pub hook_count: usize,

    /// Whether timestamps are enabled
    pub has_timestamps: bool,

    /// Whether migration was generated
    pub has_migration: bool,
}

impl GeneratedModel {
    /// Create a new generated model
    pub fn new(
        model_name: String,
        code: String,
        file_path: String,
        migration: Option<GeneratedMigration>,
        field_count: usize,
        relationship_count: usize,
        validation_count: usize,
        hook_count: usize,
        has_timestamps: bool,
    ) -> Self {
        Self {
            model_name,
            code,
            file_path,
            migration: migration.clone(),
            metadata: GenerationMetadata {
                generated_at: chrono::Utc::now(),
                generator_version: env!("CARGO_PKG_VERSION").to_string(),
                field_count,
                relationship_count,
                validation_count,
                hook_count,
                has_timestamps,
                has_migration: migration.is_some(),
            },
        }
    }

    /// Get a summary of the generated model
    pub fn summary(&self) -> String {
        format!(
            "Generated model '{}' with {} fields, {} relationships, {} validations, {} hooks",
            self.model_name,
            self.metadata.field_count,
            self.metadata.relationship_count,
            self.metadata.validation_count,
            self.metadata.hook_count
        )
    }
}

impl GeneratedMigration {
    /// Create a new generated migration
    pub fn new(name: String, up_code: String, down_code: String, file_path: String) -> Self {
        Self {
            name,
            up_code,
            down_code,
            file_path,
        }
    }
}
