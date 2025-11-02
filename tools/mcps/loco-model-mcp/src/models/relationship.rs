// Relationship definition data structures

use serde::{Deserialize, Serialize};

/// Types of relationships between models
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum RelationshipDefinition {
    /// Many-to-one relationship (this model belongs to another)
    BelongsTo {
        /// Name of the relationship (e.g., "user", "author")
        name: String,
        /// Target model name
        model: String,
        /// Foreign key column name (defaults to "{name}_id")
        foreign_key: Option<String>,
        /// Optional condition
        condition: Option<String>,
    },

    /// One-to-many relationship (this model has many of another)
    HasMany {
        /// Name of the relationship (e.g., "posts", "comments")
        name: String,
        /// Target model name
        model: String,
        /// Foreign key in the target model
        foreign_key: Option<String>,
        /// Optional condition
        condition: Option<String>,
    },

    /// One-to-one relationship
    HasOne {
        /// Name of the relationship (e.g., "profile", "settings")
        name: String,
        /// Target model name
        model: String,
        /// Foreign key in the target model
        foreign_key: Option<String>,
        /// Optional condition
        condition: Option<String>,
    },

    /// Many-to-many relationship with join table
    ManyToMany {
        /// Name of the relationship (e.g., "tags", "categories")
        name: String,
        /// Target model name
        model: String,
        /// Join table name
        join_table: String,
        /// Foreign key for this model in join table
        foreign_key: Option<String>,
        /// Foreign key for target model in join table
        association_foreign_key: Option<String>,
    },
}

impl RelationshipDefinition {
    /// Get the relationship name
    pub fn name(&self) -> &str {
        match self {
            Self::BelongsTo { name, .. }
            | Self::HasMany { name, .. }
            | Self::HasOne { name, .. }
            | Self::ManyToMany { name, .. } => name,
        }
    }

    /// Get the target model name
    pub fn model(&self) -> &str {
        match self {
            Self::BelongsTo { model, .. }
            | Self::HasMany { model, .. }
            | Self::HasOne { model, .. }
            | Self::ManyToMany { model, .. } => model,
        }
    }

    /// Get the foreign key column name (with default if not specified)
    pub fn get_foreign_key(&self) -> String {
        match self {
            Self::BelongsTo {
                name, foreign_key, ..
            } => foreign_key
                .clone()
                .unwrap_or_else(|| format!("{}_id", name)),
            Self::HasMany { foreign_key, .. } | Self::HasOne { foreign_key, .. } => foreign_key
                .clone()
                .unwrap_or_else(|| "id".to_string()),
            Self::ManyToMany { foreign_key, .. } => {
                foreign_key.clone().unwrap_or_else(|| "id".to_string())
            }
        }
    }

    /// Check if this is a belongs_to relationship
    pub fn is_belongs_to(&self) -> bool {
        matches!(self, Self::BelongsTo { .. })
    }

    /// Check if this is a has_many relationship
    pub fn is_has_many(&self) -> bool {
        matches!(self, Self::HasMany { .. })
    }

    /// Check if this is a has_one relationship
    pub fn is_has_one(&self) -> bool {
        matches!(self, Self::HasOne { .. })
    }

    /// Check if this is a many_to_many relationship
    pub fn is_many_to_many(&self) -> bool {
        matches!(self, Self::ManyToMany { .. })
    }
}
