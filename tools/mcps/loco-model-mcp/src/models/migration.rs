// Migration definition data structures

use serde::{Deserialize, Serialize};

use super::field::FieldDefinition;

/// Migration definition for database schema changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationDefinition {
    /// Migration name (usually timestamp + description)
    pub name: String,

    /// Table name to create/modify
    pub table_name: String,

    /// Migration type
    pub migration_type: MigrationType,

    /// Fields to add/modify (for create_table and add_column)
    #[serde(default)]
    pub fields: Vec<FieldDefinition>,

    /// Optional description
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// Types of migrations
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum MigrationType {
    /// Create a new table
    CreateTable,

    /// Drop an existing table
    DropTable,

    /// Add a column to an existing table
    AddColumn,

    /// Remove a column from an existing table
    RemoveColumn,

    /// Modify an existing column
    ModifyColumn,

    /// Rename a column
    RenameColumn { from: String, to: String },

    /// Add an index
    AddIndex {
        columns: Vec<String>,
        unique: bool,
    },

    /// Remove an index
    RemoveIndex { name: String },

    /// Custom migration SQL
    Custom { up_sql: String, down_sql: String },
}

impl MigrationDefinition {
    /// Create a new migration definition
    pub fn new(name: String, table_name: String, migration_type: MigrationType) -> Self {
        Self {
            name,
            table_name,
            migration_type,
            fields: Vec::new(),
            description: None,
        }
    }

    /// Create a create_table migration
    pub fn create_table(table_name: String, fields: Vec<FieldDefinition>) -> Self {
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
        let name = format!("m{}create{}", timestamp, &table_name);
        let description = format!("Create {} table", &table_name);

        Self {
            name,
            table_name,
            migration_type: MigrationType::CreateTable,
            fields,
            description: Some(description),
        }
    }

    /// Create a drop_table migration
    pub fn drop_table(table_name: String) -> Self {
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
        let name = format!("m{}drop{}", timestamp, &table_name);
        let description = format!("Drop {} table", &table_name);

        Self {
            name,
            table_name,
            migration_type: MigrationType::DropTable,
            fields: Vec::new(),
            description: Some(description),
        }
    }

    /// Create an add_column migration
    pub fn add_column(table_name: String, field: FieldDefinition) -> Self {
        let timestamp = chrono::Utc::now().format("%Y%m%d%H%M%S").to_string();
        let name = format!("m{}add{}to{}", timestamp, &field.name, &table_name);
        let description = format!("Add column to {} table", &table_name);

        Self {
            name,
            table_name,
            migration_type: MigrationType::AddColumn,
            fields: vec![field],
            description: Some(description),
        }
    }

    /// Generate migration filename
    pub fn filename(&self) -> String {
        format!("{}.rs", self.name)
    }
}
