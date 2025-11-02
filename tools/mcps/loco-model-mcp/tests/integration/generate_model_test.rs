// Integration tests for generate_model MCP tool
// These tests verify the full MCP request/response flow

use serde_json::json;

/// Test the generate_model tool with a basic model specification
#[tokio::test]
async fn test_generate_model_basic() {
    // Arrange: Create a basic model request
    let request = json!({
        "name": "User",
        "fields": [
            {"name": "email", "type": "string", "nullable": false, "unique": true},
            {"name": "name", "type": "string", "nullable": false},
            {"name": "age", "type": "integer", "nullable": true}
        ],
        "timestamps": true,
        "generate_migration": true
    });

    // Act: Call the generate_model tool
    // TODO: This will be implemented in T031 when we add MCP tool handler
    let result = call_generate_model_tool(request).await;

    // Assert: Verify the response structure
    assert!(result.is_ok(), "Tool should return success");
    let response = result.unwrap();

    // Verify response contains generated model code
    assert!(response.get("model_code").is_some(), "Response should contain model_code");
    let model_code = response["model_code"].as_str().unwrap();

    // Basic structure checks
    assert!(model_code.contains("pub struct Model"), "Should define Model struct");
    assert!(model_code.contains("DeriveEntityModel"), "Should use DeriveEntityModel");
    assert!(model_code.contains("email"), "Should include email field");
    assert!(model_code.contains("name"), "Should include name field");
    assert!(model_code.contains("age"), "Should include age field");

    // Verify migration is included when requested
    assert!(response.get("migration_code").is_some(), "Response should contain migration_code");
}

#[tokio::test]
async fn test_generate_model_without_timestamps() {
    // Arrange: Model without timestamps
    let request = json!({
        "name": "Config",
        "fields": [
            {"name": "key", "type": "string", "nullable": false, "unique": true},
            {"name": "value", "type": "text", "nullable": true}
        ],
        "timestamps": false,
        "generate_migration": false
    });

    // Act
    let result = call_generate_model_tool(request).await;

    // Assert
    assert!(result.is_ok(), "Tool should succeed without timestamps");
    let response = result.unwrap();
    let model_code = response["model_code"].as_str().unwrap();

    // Should not include created_at/updated_at
    assert!(!model_code.contains("created_at"), "Should not have created_at");
    assert!(!model_code.contains("updated_at"), "Should not have updated_at");

    // Migration should not be present
    assert!(response.get("migration_code").is_none(), "Should not contain migration_code");
}

#[tokio::test]
async fn test_generate_model_with_relationships() {
    // Arrange: Model with relationships
    let request = json!({
        "name": "Post",
        "fields": [
            {"name": "title", "type": "string", "nullable": false},
            {"name": "content", "type": "text", "nullable": false},
            {"name": "user_id", "type": "integer", "nullable": false}
        ],
        "relationships": [
            {
                "type": "belongs_to",
                "name": "user",
                "model": "User",
                "foreign_key": "user_id"
            }
        ],
        "timestamps": true,
        "generate_migration": true
    });

    // Act
    let result = call_generate_model_tool(request).await;

    // Assert
    assert!(result.is_ok(), "Tool should handle relationships");
    let response = result.unwrap();
    let model_code = response["model_code"].as_str().unwrap();

    // Verify relationship code is included
    assert!(model_code.contains("user"), "Should include user relationship");
    assert!(model_code.contains("user_id"), "Should include foreign key field");
}

#[tokio::test]
async fn test_generate_model_with_validations() {
    // Arrange: Model with validation rules
    let request = json!({
        "name": "Account",
        "fields": [
            {"name": "email", "type": "string", "nullable": false, "unique": true},
            {"name": "username", "type": "string", "nullable": false}
        ],
        "validations": [
            {
                "field": "email",
                "rule": {"type": "email"}
            },
            {
                "field": "username",
                "rule": {
                    "type": "length",
                    "min": 3,
                    "max": 20
                }
            }
        ],
        "timestamps": true,
        "generate_migration": true
    });

    // Act
    let result = call_generate_model_tool(request).await;

    // Assert
    assert!(result.is_ok(), "Tool should handle validations");
    let response = result.unwrap();
    let model_code = response["model_code"].as_str().unwrap();

    // Verify validation attributes are present
    assert!(model_code.contains("#[validate"), "Should include validation attributes");
}

#[tokio::test]
async fn test_generate_model_invalid_name() {
    // Arrange: Invalid model name
    let request = json!({
        "name": "",  // Empty name should fail validation
        "fields": [
            {"name": "field1", "type": "string"}
        ]
    });

    // Act
    let result = call_generate_model_tool(request).await;

    // Assert: Should return validation error
    assert!(result.is_err(), "Should reject empty model name");
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Invalid model name") ||
            error.to_string().contains("empty"),
            "Error should mention invalid name");
}

#[tokio::test]
async fn test_generate_model_no_fields() {
    // Arrange: Model with no fields
    let request = json!({
        "name": "Empty",
        "fields": []  // No fields should fail validation
    });

    // Act
    let result = call_generate_model_tool(request).await;

    // Assert: Should return validation error
    assert!(result.is_err(), "Should reject model with no fields");
    let error = result.unwrap_err();
    assert!(error.to_string().contains("field") ||
            error.to_string().contains("at least one"),
            "Error should mention missing fields");
}

#[tokio::test]
async fn test_generate_model_duplicate_fields() {
    // Arrange: Model with duplicate field names
    let request = json!({
        "name": "Invalid",
        "fields": [
            {"name": "email", "type": "string"},
            {"name": "email", "type": "string"}  // Duplicate!
        ]
    });

    // Act
    let result = call_generate_model_tool(request).await;

    // Assert: Should return validation error
    assert!(result.is_err(), "Should reject duplicate field names");
    let error = result.unwrap_err();
    assert!(error.to_string().contains("Duplicate") ||
            error.to_string().contains("duplicate"),
            "Error should mention duplicate field");
}

#[tokio::test]
async fn test_generate_model_custom_table_name() {
    // Arrange: Model with custom table name
    let request = json!({
        "name": "User",
        "table_name": "app_users",  // Custom table name
        "fields": [
            {"name": "email", "type": "string", "nullable": false}
        ],
        "timestamps": true,
        "generate_migration": true
    });

    // Act
    let result = call_generate_model_tool(request).await;

    // Assert
    assert!(result.is_ok(), "Tool should handle custom table name");
    let response = result.unwrap();
    let model_code = response["model_code"].as_str().unwrap();

    // Verify custom table name is used
    assert!(model_code.contains("app_users"), "Should use custom table name");

    // Migration should also use custom table name
    let migration_code = response["migration_code"].as_str().unwrap();
    assert!(migration_code.contains("app_users"), "Migration should use custom table name");
}

// Helper function that will call the actual MCP tool when implemented
async fn call_generate_model_tool(
    request: serde_json::Value,
) -> Result<serde_json::Value, loco_model_mcp::error::LocoModelError> {
    // TODO: This will be implemented in T031 when we add the MCP tool handler
    // For now, return an error to make tests fail as expected in TDD
    Err(loco_model_mcp::error::LocoModelError::McpProtocol(
        "MCP tool not yet implemented".to_string(),
    ))
}
