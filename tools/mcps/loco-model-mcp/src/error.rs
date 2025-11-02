// Error types for Loco model generation

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LocoModelError {
    // Validation errors
    #[error("Invalid model name: {0}")]
    InvalidModelName(String),

    #[error("Invalid field name: {0}")]
    InvalidFieldName(String),

    #[error("Invalid field type: {0}")]
    InvalidFieldType(String),

    #[error("Duplicate field: {0}")]
    DuplicateField(String),

    #[error("Invalid relationship: {0}")]
    InvalidRelationship(String),

    #[error("Invalid validation rule: {0}")]
    InvalidValidation(String),

    #[error("Invalid hook: {0}")]
    InvalidHook(String),

    #[error("Validation error: {0}")]
    Validation(String),

    // Generation errors
    #[error("Code generation failed: {0}")]
    Generation(String),

    #[error("Template rendering failed: {0}")]
    TemplateRendering(String),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),

    // File system errors
    #[error("File system error: {0}")]
    FileSystem(#[from] std::io::Error),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("File already exists: {0}")]
    FileExists(String),

    // Serialization errors
    #[error("JSON serialization error: {0}")]
    JsonSerialization(#[from] serde_json::Error),

    // Template errors
    #[error("Template error: {0}")]
    Template(String),

    // MCP-specific errors
    #[error("MCP protocol error: {0}")]
    McpProtocol(String),

    #[error("Invalid tool parameters: {0}")]
    InvalidToolParams(String),

    #[error("MCP server error: {0}")]
    Mcp(String),
}

pub type Result<T> = std::result::Result<T, LocoModelError>;
