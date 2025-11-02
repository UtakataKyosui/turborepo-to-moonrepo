// Snapshot tests for basic model code generation
// These tests use insta for snapshot testing to ensure generated code stability

use loco_model_mcp::models::{
    field::{FieldDefinition, LocoFieldType},
    specification::ModelSpecification,
};

/// Helper function to generate model code from specification
/// This will call the actual generator when implemented
fn generate_model_code(spec: &ModelSpecification) -> Result<String, loco_model_mcp::error::LocoModelError> {
    // TODO: This will be implemented in T030
    // For now, this will cause tests to fail as expected in TDD
    Err(loco_model_mcp::error::LocoModelError::Generation(
        "Model generator not yet implemented".to_string(),
    ))
}

#[test]
fn test_basic_model_with_simple_fields() {
    // Arrange: Create a basic User model with simple fields
    let spec = ModelSpecification::new(
        "User".to_string(),
        vec![
            FieldDefinition::new("email".to_string(), LocoFieldType::String),
            FieldDefinition::new("name".to_string(), LocoFieldType::String),
            FieldDefinition::new("age".to_string(), LocoFieldType::Integer),
        ],
    );

    // Act: Generate the model code
    let result = generate_model_code(&spec);

    // Assert: Snapshot test the generated code
    // This will FAIL until T030 is implemented
    assert!(result.is_ok(), "Model generation should succeed");
    insta::assert_snapshot!("basic_user_model", result.unwrap());
}

#[test]
fn test_model_with_timestamps_enabled() {
    // Arrange: Create a Post model with timestamps (default enabled)
    let spec = ModelSpecification {
        name: "Post".to_string(),
        table_name: None,
        fields: vec![
            FieldDefinition::new("title".to_string(), LocoFieldType::String),
            FieldDefinition::new("content".to_string(), LocoFieldType::Text),
        ],
        relationships: vec![],
        validations: vec![],
        hooks: vec![],
        timestamps: true, // Should generate created_at and updated_at
        generate_migration: true,
    };

    // Act
    let result = generate_model_code(&spec);

    // Assert
    assert!(result.is_ok(), "Model generation with timestamps should succeed");
    insta::assert_snapshot!("post_model_with_timestamps", result.unwrap());
}

#[test]
fn test_model_with_timestamps_disabled() {
    // Arrange: Create a Config model without timestamps
    let spec = ModelSpecification {
        name: "Config".to_string(),
        table_name: None,
        fields: vec![
            FieldDefinition::new("key".to_string(), LocoFieldType::String),
            FieldDefinition::new("value".to_string(), LocoFieldType::Text),
        ],
        relationships: vec![],
        validations: vec![],
        hooks: vec![],
        timestamps: false, // No created_at/updated_at
        generate_migration: true,
    };

    // Act
    let result = generate_model_code(&spec);

    // Assert
    assert!(result.is_ok(), "Model generation without timestamps should succeed");
    insta::assert_snapshot!("config_model_without_timestamps", result.unwrap());
}

#[test]
fn test_model_with_various_field_types() {
    // Arrange: Create a comprehensive model with all common field types
    let spec = ModelSpecification::new(
        "Article".to_string(),
        vec![
            FieldDefinition::new("title".to_string(), LocoFieldType::String),
            FieldDefinition::new("slug".to_string(), LocoFieldType::String),
            FieldDefinition::new("content".to_string(), LocoFieldType::Text),
            FieldDefinition::new("view_count".to_string(), LocoFieldType::Integer),
            FieldDefinition::new("is_published".to_string(), LocoFieldType::Boolean),
            FieldDefinition::new("published_at".to_string(), LocoFieldType::Timestamp),
            FieldDefinition::new("uuid".to_string(), LocoFieldType::Uuid),
            FieldDefinition::new("rating".to_string(), LocoFieldType::Float),
            FieldDefinition::new("metadata".to_string(), LocoFieldType::Jsonb),
        ],
    );

    // Act
    let result = generate_model_code(&spec);

    // Assert
    assert!(result.is_ok(), "Model generation with various types should succeed");
    insta::assert_snapshot!("article_model_various_types", result.unwrap());
}

#[test]
fn test_model_with_nullable_fields() {
    // Arrange: Model with mix of required and nullable fields
    let mut spec = ModelSpecification::new(
        "Profile".to_string(),
        vec![
            {
                let mut field = FieldDefinition::new("user_id".to_string(), LocoFieldType::Integer);
                field.nullable = false; // Required
                field
            },
            {
                let mut field = FieldDefinition::new("bio".to_string(), LocoFieldType::Text);
                field.nullable = true; // Optional
                field
            },
            {
                let mut field = FieldDefinition::new("avatar_url".to_string(), LocoFieldType::String);
                field.nullable = true; // Optional
                field
            },
        ],
    );
    spec.timestamps = true;

    // Act
    let result = generate_model_code(&spec);

    // Assert
    assert!(result.is_ok(), "Model generation with nullable fields should succeed");
    insta::assert_snapshot!("profile_model_nullable_fields", result.unwrap());
}

#[test]
fn test_model_with_unique_fields() {
    // Arrange: Model with unique constraints
    let spec = ModelSpecification::new(
        "Account".to_string(),
        vec![
            {
                let mut field = FieldDefinition::new("email".to_string(), LocoFieldType::String);
                field.unique = true; // Must be unique
                field.nullable = false;
                field
            },
            {
                let mut field = FieldDefinition::new("username".to_string(), LocoFieldType::String);
                field.unique = true; // Must be unique
                field.nullable = false;
                field
            },
            FieldDefinition::new("password_hash".to_string(), LocoFieldType::String),
        ],
    );

    // Act
    let result = generate_model_code(&spec);

    // Assert
    assert!(result.is_ok(), "Model generation with unique fields should succeed");
    insta::assert_snapshot!("account_model_unique_fields", result.unwrap());
}

#[test]
fn test_model_with_custom_table_name() {
    // Arrange: Model with explicit custom table name
    let spec = ModelSpecification {
        name: "User".to_string(),
        table_name: Some("app_users".to_string()), // Custom table name instead of "users"
        fields: vec![
            FieldDefinition::new("email".to_string(), LocoFieldType::String),
            FieldDefinition::new("name".to_string(), LocoFieldType::String),
        ],
        relationships: vec![],
        validations: vec![],
        hooks: vec![],
        timestamps: true,
        generate_migration: true,
    };

    // Act
    let result = generate_model_code(&spec);

    // Assert
    assert!(result.is_ok(), "Model generation with custom table name should succeed");
    insta::assert_snapshot!("user_model_custom_table_name", result.unwrap());
}

#[test]
fn test_model_with_indexed_fields() {
    // Arrange: Model with database indexes
    let spec = ModelSpecification::new(
        "Product".to_string(),
        vec![
            {
                let mut field = FieldDefinition::new("sku".to_string(), LocoFieldType::String);
                field.indexed = true; // Create index for fast lookups
                field.unique = true;
                field
            },
            {
                let mut field = FieldDefinition::new("category".to_string(), LocoFieldType::String);
                field.indexed = true; // Index for filtering
                field
            },
            FieldDefinition::new("name".to_string(), LocoFieldType::String),
            FieldDefinition::new("price".to_string(), LocoFieldType::Decimal {
                precision: 10,
                scale: 2,
            }),
        ],
    );

    // Act
    let result = generate_model_code(&spec);

    // Assert
    assert!(result.is_ok(), "Model generation with indexed fields should succeed");
    insta::assert_snapshot!("product_model_indexed_fields", result.unwrap());
}

#[test]
fn test_model_code_structure() {
    // Arrange: Simple model to verify overall code structure
    let spec = ModelSpecification::new(
        "Task".to_string(),
        vec![
            FieldDefinition::new("title".to_string(), LocoFieldType::String),
            FieldDefinition::new("completed".to_string(), LocoFieldType::Boolean),
        ],
    );

    // Act
    let result = generate_model_code(&spec);

    // Assert: Verify the generated code has expected structure
    assert!(result.is_ok(), "Basic model generation should succeed");
    let code = result.unwrap();

    // These assertions will help verify structure when implemented
    assert!(code.contains("use sea_orm::entity::prelude::*;"), "Should include SeaORM imports");
    assert!(code.contains("DeriveEntityModel"), "Should use DeriveEntityModel macro");
    assert!(code.contains("pub struct Model"), "Should define Model struct");
    assert!(code.contains("#[sea_orm("), "Should have SeaORM attributes");

    insta::assert_snapshot!("task_model_structure", code);
}

#[test]
fn test_loco_field_suffix_notation() {
    // Arrange: Test that field suffixes are correctly applied
    let spec = ModelSpecification::new(
        "Example".to_string(),
        vec![
            {
                let mut field = FieldDefinition::new("required_field".to_string(), LocoFieldType::String);
                field.nullable = false; // Should get ! suffix
                field
            },
            {
                let mut field = FieldDefinition::new("unique_field".to_string(), LocoFieldType::String);
                field.unique = true; // Should get ^ suffix
                field
            },
            {
                let mut field = FieldDefinition::new("optional_field".to_string(), LocoFieldType::String);
                field.nullable = true; // No suffix
                field
            },
        ],
    );

    // Act
    let result = generate_model_code(&spec);

    // Assert
    assert!(result.is_ok(), "Model with various suffixes should generate");

    // Verify suffix logic is correct
    for field in &spec.fields {
        let suffix = field.get_loco_suffix();
        match field.name.as_str() {
            "required_field" => assert_eq!(suffix, "!", "Required fields should have ! suffix"),
            "unique_field" => assert_eq!(suffix, "^", "Unique fields should have ^ suffix"),
            "optional_field" => assert_eq!(suffix, "", "Nullable fields should have no suffix"),
            _ => {}
        }
    }

    insta::assert_snapshot!("example_model_field_suffixes", result.unwrap());
}
