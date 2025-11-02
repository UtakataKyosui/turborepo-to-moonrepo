# Data Model: Loco Model MCP Tool

**Feature**: 001-loco-model-mcp
**Date**: 2025-10-19
**Phase**: Phase 1 - Design

## Overview

This document defines the internal data structures used by the Loco Model MCP tool for processing model specifications and generating code. These entities represent the domain model of the code generation system.

---

## Core Entities

### 1. ModelSpecification

**Purpose**: Represents a user's complete model definition request parsed from natural language or JSON input.

**Attributes**:
```rust
pub struct ModelSpecification {
    /// Model name in PascalCase (e.g., "User", "BlogPost")
    pub name: String,

    /// Table name in snake_case (auto-derived if not specified)
    pub table_name: Option<String>,

    /// List of field definitions
    pub fields: Vec<FieldDefinition>,

    /// Relationship definitions with other models
    pub relationships: Vec<RelationshipDefinition>,

    /// Validation rules to apply
    pub validations: Vec<ValidationRule>,

    /// Lifecycle hooks to implement
    pub hooks: Vec<LifecycleHook>,

    /// Whether to auto-generate timestamps (created_at, updated_at)
    pub timestamps: bool,

    /// Whether to generate corresponding migration
    pub generate_migration: bool,
}
```

**Validation Rules**:
- `name` must be non-empty and PascalCase
- `name` must not conflict with Rust reserved keywords
- `fields` must have at least one field
- `fields` must not contain duplicate names
- At least one field should be suitable as primary key (or id field is auto-added)

**State Transitions**:
```
Created → Validated → CodeGenerated → Written
```

---

### 2. FieldDefinition

**Purpose**: Describes a single field/column in the model with its type, constraints, and metadata.

**Attributes**:
```rust
pub struct FieldDefinition {
    /// Field name in snake_case (e.g., "email", "created_at")
    pub name: String,

    /// Loco field type (string, int, uuid, etc.)
    pub field_type: LocoFieldType,

    /// Whether the field can be NULL in database
    pub nullable: bool,

    /// Whether the field has UNIQUE constraint
    pub unique: bool,

    /// Whether this field is the primary key
    pub primary_key: bool,

    /// Default value expression (optional)
    pub default_value: Option<DefaultValue>,

    /// Whether to create database index on this field
    pub indexed: bool,

    /// Field-level validation constraints
    pub validations: Vec<FieldValidation>,

    /// Documentation comment for the field
    pub description: Option<String>,
}
```

**LocoFieldType Enum**:
```rust
pub enum LocoFieldType {
    String,
    Text,
    Integer,
    SmallInt,
    BigInt,
    Uuid,
    Boolean,
    Timestamp,
    Date,
    Time,
    Json,
    Jsonb,
    Float,
    Double,
    Decimal { precision: u32, scale: u32 },
    Binary,
    Reference { model: String, column: Option<String> },
}
```

**Rust Type Mapping**:
```rust
impl LocoFieldType {
    pub fn to_rust_type(&self, nullable: bool) -> String {
        let base_type = match self {
            LocoFieldType::String | LocoFieldType::Text => "String",
            LocoFieldType::Integer => "i32",
            LocoFieldType::SmallInt => "i16",
            LocoFieldType::BigInt => "i64",
            LocoFieldType::Uuid => "Uuid",
            LocoFieldType::Boolean => "bool",
            LocoFieldType::Timestamp => "DateTimeWithTimeZone",
            LocoFieldType::Date => "Date",
            LocoFieldType::Time => "Time",
            LocoFieldType::Json | LocoFieldType::Jsonb => "serde_json::Value",
            LocoFieldType::Float => "f32",
            LocoFieldType::Double => "f64",
            LocoFieldType::Decimal { .. } => "Decimal",
            LocoFieldType::Binary => "Vec<u8>",
            LocoFieldType::Reference { model, .. } => return format!("{}Id", model),
        };

        if nullable {
            format!("Option<{}>", base_type)
        } else {
            base_type.to_string()
        }
    }
}
```

**Validation Rules**:
- `name` must be snake_case
- `name` must not be a Rust reserved keyword
- Primary key fields must be non-nullable
- Reference fields must point to valid model names

---

### 3. RelationshipDefinition

**Purpose**: Defines associations between models (belongs_to, has_many, has_one, many_to_many).

**Attributes**:
```rust
pub enum RelationshipDefinition {
    BelongsTo {
        /// Target model name (e.g., "User")
        model: String,
        /// Foreign key field name (e.g., "user_id")
        foreign_key: String,
        /// Target primary key field (defaults to "id")
        primary_key: Option<String>,
        /// Optional alias for the relationship
        alias: Option<String>,
        /// Cascade on update/delete
        on_update: CascadeAction,
        on_delete: CascadeAction,
    },

    HasMany {
        /// Target model name (plural, e.g., "Posts")
        model: String,
        /// Foreign key in target model
        foreign_key: String,
        /// Optional alias for the relationship
        alias: Option<String>,
    },

    HasOne {
        /// Target model name (e.g., "Profile")
        model: String,
        /// Foreign key in target model
        foreign_key: String,
        /// Optional alias for the relationship
        alias: Option<String>,
    },

    ManyToMany {
        /// Target model name (e.g., "Tags")
        model: String,
        /// Join table name (e.g., "post_tags")
        join_table: String,
        /// Foreign key in join table for this model
        foreign_key: String,
        /// Foreign key in join table for target model
        association_foreign_key: String,
    },
}

pub enum CascadeAction {
    Cascade,
    Restrict,
    SetNull,
    NoAction,
}
```

**SeaORM Mapping**:
```rust
// BelongsTo example
#[sea_orm(
    belongs_to = "super::users::Entity",
    from = "Column::UserId",
    to = "super::users::Column::Id",
    on_update = "Cascade",
    on_delete = "Cascade"
)]

// HasMany example
#[sea_orm(has_many = "super::posts::Entity")]

// ManyToMany via join table
fn via() -> Option<RelationDef> {
    Some(super::post_tags::Relation::Post.def().rev())
}
```

**Validation Rules**:
- Target model must exist or be defined in same generation batch
- Foreign keys must reference valid fields
- Join tables for many_to_many must have both foreign keys
- Circular dependencies should be detected and warned

---

### 4. ValidationRule

**Purpose**: Defines validation constraints to be enforced at application level using the `validator` crate.

**Attributes**:
```rust
pub struct ValidationRule {
    /// Field this validation applies to
    pub field: String,

    /// Type of validation
    pub rule_type: ValidationType,

    /// Custom error message (optional)
    pub message: Option<String>,
}

pub enum ValidationType {
    /// Email format validation
    Email,

    /// URL format validation
    Url,

    /// String length constraints
    Length {
        min: Option<usize>,
        max: Option<usize>,
    },

    /// Numeric range constraints
    Range {
        min: Option<f64>,
        max: Option<f64>,
    },

    /// Regular expression pattern
    Regex {
        pattern: String,
    },

    /// Must be true (for checkboxes, terms acceptance)
    MustBeTrue,

    /// Custom validation function
    Custom {
        function_name: String,
    },

    /// Required field (non-null, non-empty)
    Required,
}
```

**Code Generation Example**:
```rust
// Input ValidationRule
ValidationRule {
    field: "email".to_string(),
    rule_type: ValidationType::Email,
    message: Some("Invalid email address".to_string()),
}

// Generated code
#[validate(email(message = "Invalid email address"))]
pub email: String,
```

---

### 5. LifecycleHook

**Purpose**: Defines callbacks to execute at specific points in the model's lifecycle (create, update, delete).

**Attributes**:
```rust
pub struct LifecycleHook {
    /// Type of hook
    pub hook_type: HookType,

    /// Custom implementation code
    pub implementation: HookImplementation,
}

pub enum HookType {
    BeforeSave,
    AfterSave,
    BeforeCreate,
    AfterCreate,
    BeforeUpdate,
    AfterUpdate,
    BeforeDelete,
    AfterDelete,
}

pub enum HookImplementation {
    /// Predefined common operations
    Preset(PresetHook),

    /// Custom Rust code
    Custom(String),
}

pub enum PresetHook {
    /// Normalize email to lowercase
    NormalizeEmail { field: String },

    /// Generate UUID for field
    GenerateUuid { field: String },

    /// Set timestamp to now
    SetTimestamp { field: String },

    /// Trim whitespace from string fields
    TrimWhitespace { fields: Vec<String> },

    /// Hash password field
    HashPassword { field: String, algorithm: String },
}
```

**SeaORM ActiveModelBehavior Mapping**:
```rust
#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        // HookType::BeforeSave implementation
        if let Set(email) = &self.email {
            self.email = Set(email.to_lowercase());  // NormalizeEmail preset
        }
        Ok(self)
    }
}
```

---

### 6. GeneratedModel

**Purpose**: Container for the complete generated code output including model, migrations, and metadata.

**Attributes**:
```rust
pub struct GeneratedModel {
    /// Generated Rust entity code
    pub entity_code: String,

    /// Generated custom model code (validations, hooks)
    pub custom_model_code: String,

    /// Generated migration code (if requested)
    pub migration_code: Option<String>,

    /// File path where entity should be written
    pub entity_path: PathBuf,

    /// File path where custom model should be written
    pub custom_model_path: PathBuf,

    /// File path where migration should be written (if applicable)
    pub migration_path: Option<PathBuf>,

    /// Metadata about the generation
    pub metadata: GenerationMetadata,
}

pub struct GenerationMetadata {
    /// Timestamp of generation
    pub generated_at: DateTime<Utc>,

    /// Generator version
    pub generator_version: String,

    /// Source specification hash (for caching)
    pub spec_hash: String,

    /// Compilation status
    pub compiled: bool,

    /// Any warnings generated during code generation
    pub warnings: Vec<String>,
}
```

**Output Structure**:
```
src/
├── models/
│   ├── _entities/
│   │   └── users.rs          # entity_code (auto-generated, don't edit)
│   └── users.rs              # custom_model_code (manual additions)
└── ...

migration/
└── m20250119_create_users.rs  # migration_code
```

---

### 7. MigrationDefinition

**Purpose**: Describes database migration to create the table schema matching the model.

**Attributes**:
```rust
pub struct MigrationDefinition {
    /// Migration name (e.g., "create_users")
    pub name: String,

    /// Table name to create
    pub table_name: String,

    /// Columns to create
    pub columns: Vec<ColumnDefinition>,

    /// Indexes to create
    pub indexes: Vec<IndexDefinition>,

    /// Foreign key constraints
    pub foreign_keys: Vec<ForeignKeyDefinition>,
}

pub struct ColumnDefinition {
    pub name: String,
    pub column_type: ColumnType,
    pub nullable: bool,
    pub unique: bool,
    pub primary_key: bool,
    pub auto_increment: bool,
    pub default: Option<String>,
}

pub enum ColumnType {
    Integer,
    BigInteger,
    SmallInteger,
    String(Option<u32>),  // VARCHAR with optional length
    Text,
    Uuid,
    Boolean,
    Timestamp,
    TimestampTz,
    Date,
    Time,
    Json,
    Jsonb,
    Float,
    Double,
    Decimal(u32, u32),  // precision, scale
    Binary,
}

pub struct IndexDefinition {
    pub name: String,
    pub columns: Vec<String>,
    pub unique: bool,
    pub index_type: Option<IndexType>,
}

pub enum IndexType {
    BTree,
    Hash,
    Gist,
    Gin,
}

pub struct ForeignKeyDefinition {
    pub name: String,
    pub columns: Vec<String>,
    pub referenced_table: String,
    pub referenced_columns: Vec<String>,
    pub on_delete: CascadeAction,
    pub on_update: CascadeAction,
}
```

**SeaORM Migration Generation**:
```rust
manager
    .create_table(
        Table::create()
            .table(Users::Table)
            .col(ColumnDef::new(Users::Id).integer().auto_increment().primary_key())
            .col(ColumnDef::new(Users::Email).string().not_null().unique_key())
            .col(ColumnDef::new(Users::Name).string().not_null())
            .to_owned()
    )
    .await?;
```

---

## Entity Relationships Diagram

```
┌─────────────────────┐
│ ModelSpecification  │
│                     │
│ - name              │
│ - fields            │◄──────┐
│ - relationships     │       │
│ - validations       │       │
│ - hooks             │       │
└─────────────────────┘       │
         │                     │
         │ 1                   │
         │                     │
         │ *                   │
         ▼                     │
┌─────────────────────┐       │
│ FieldDefinition     │       │
│                     │       │
│ - name              │       │
│ - field_type        │───────┘
│ - nullable          │
│ - unique            │
│ - validations       │◄──────┐
└─────────────────────┘       │
         │                     │
         │ 1                   │
         │                     │
         │ *                   │
         ▼                     │
┌─────────────────────┐       │
│ ValidationRule      │       │
│                     │       │
│ - field             │───────┘
│ - rule_type         │
│ - message           │
└─────────────────────┘

┌─────────────────────┐
│ ModelSpecification  │
└─────────────────────┘
         │
         │ 1
         │
         │ *
         ▼
┌─────────────────────┐
│ RelationshipDef     │
│                     │
│ - model             │
│ - foreign_key       │
│ - cascade_actions   │
└─────────────────────┘

┌─────────────────────┐
│ ModelSpecification  │
└─────────────────────┘
         │
         │ 1
         │
         │ *
         ▼
┌─────────────────────┐
│ LifecycleHook       │
│                     │
│ - hook_type         │
│ - implementation    │
└─────────────────────┘

┌─────────────────────┐
│ ModelSpecification  │
└─────────────────────┘
         │
         │ 1
         │
         │ 1
         ▼
┌─────────────────────┐       ┌─────────────────────┐
│ GeneratedModel      │──────►│ MigrationDefinition │
│                     │  0..1  │                     │
│ - entity_code       │       │ - columns           │
│ - migration_code    │       │ - indexes           │
│ - metadata          │       │ - foreign_keys      │
└─────────────────────┘       └─────────────────────┘
```

---

## Data Flow

```
User Input (Natural Language/JSON)
         │
         ▼
    [Parse & Validate]
         │
         ▼
  ModelSpecification
         │
         ├─→ FieldDefinition (multiple)
         ├─→ RelationshipDefinition (multiple)
         ├─→ ValidationRule (multiple)
         └─→ LifecycleHook (multiple)
         │
         ▼
   [Code Generator]
         │
         ├─→ Entity Code (Rust)
         ├─→ Custom Model Code (Rust)
         └─→ Migration Code (Rust)
         │
         ▼
   GeneratedModel
         │
         ▼
  [Write to Files]
```

---

## Validation Strategy

Each entity has validation rules enforced at different stages:

**Parse-Time Validation**:
- JSON schema compliance
- Required field presence
- Type correctness

**Semantic Validation**:
- Name conventions (PascalCase, snake_case)
- Reserved keyword conflicts
- Field type compatibility
- Relationship target existence

**Generation-Time Validation**:
- Syntax validation of generated code using `syn`
- Compilation check (optional, async)
- Loco convention compliance

---

## State Management

**Specification State Machine**:
```
   Created
      │
      ▼
  Validated ──[fail]──► Invalid
      │
      ▼
 CodeGenerated
      │
      ▼
   Written ──[success]──► Complete
      │
   [fail]
      │
      ▼
   Error
```

**Caching Strategy**:
- Cache key: Hash of ModelSpecification
- Cache value: GeneratedModel
- Eviction: LRU with 100-item capacity
- TTL: None (in-memory only, cleared on restart)

---

## Extensibility Points

**Custom Field Types**:
```rust
pub trait CustomFieldType {
    fn name(&self) -> &str;
    fn rust_type(&self, nullable: bool) -> String;
    fn database_type(&self) -> String;
    fn validation(&self) -> Option<String>;
}
```

**Custom Validation**:
```rust
pub trait CustomValidator {
    fn validate(&self, value: &serde_json::Value) -> Result<(), ValidationError>;
    fn to_code(&self) -> String;
}
```

**Template Customization**:
- Tera templates can be overridden via configuration
- Custom template directory support
- Template inheritance for base patterns

---

**Data Model Status**: ✅ COMPLETE
**Next Step**: Generate MCP tool contracts (Phase 1)
