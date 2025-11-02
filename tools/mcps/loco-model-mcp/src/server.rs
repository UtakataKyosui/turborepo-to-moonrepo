// MCP server implementation for Loco model generation

use crate::generator::model::generate_model_code;
use rmcp::handler::server::router::tool::ToolRouter;
use rmcp::handler::server::wrapper::Parameters;
use rmcp::handler::server::ServerHandler;
use rmcp::model::{
    CallToolResult, Content, ErrorCode, InitializeRequestParam, InitializeResult, ProtocolVersion,
    ServerCapabilities, ServerInfo,
};
use rmcp::service::{RequestContext, RoleServer};
use rmcp::{tool, tool_handler, tool_router, ErrorData as McpError};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Parameters for generate_model tool
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct GenerateModelParams {
    /// Name of the model (e.g., User, Post)
    pub name: String,
    /// Array of field definitions
    pub fields: Vec<Value>,
    /// Array of relationship definitions (optional)
    #[serde(default)]
    pub relationships: Vec<Value>,
    /// Custom table name (optional)
    #[serde(default)]
    pub table_name: Option<String>,
    /// Whether to include created_at/updated_at timestamps (default: true)
    #[serde(default = "default_true")]
    pub timestamps: bool,
}

fn default_true() -> bool {
    true
}

#[derive(Clone)]
pub struct LocoModelServer {
    tool_router: ToolRouter<Self>,
}

#[tool_router]
impl LocoModelServer {
    pub fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    /// Generate Loco framework model code from specification
    #[tool(description = "Generate Loco framework model code from specification")]
    async fn generate_model(
        &self,
        params: Parameters<GenerateModelParams>,
    ) -> Result<CallToolResult, McpError> {
        // Build JSON specification
        let spec = serde_json::json!({
            "name": params.0.name,
            "fields": params.0.fields,
            "relationships": params.0.relationships,
            "table_name": params.0.table_name,
            "timestamps": params.0.timestamps,
        });

        // Generate model code
        match generate_model_code(&spec) {
            Ok(code) => Ok(CallToolResult::success(vec![Content::text(code)])),
            Err(e) => Err(McpError {
                code: ErrorCode(-32000),
                message: format!("Model generation failed: {}", e).into(),
                data: None,
            }),
        }
    }
}

#[tool_handler]
impl ServerHandler for LocoModelServer {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: rmcp::model::Implementation {
                name: "loco-model-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                icons: None,
                title: None,
                website_url: None,
            },
            instructions: Some(
                "MCP server for generating Loco framework model code from specifications"
                    .to_string(),
            ),
        }
    }

    async fn initialize(
        &self,
        _params: InitializeRequestParam,
        _context: RequestContext<RoleServer>,
    ) -> Result<InitializeResult, McpError> {
        Ok(InitializeResult {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            server_info: rmcp::model::Implementation {
                name: "loco-model-mcp".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                icons: None,
                title: None,
                website_url: None,
            },
            instructions: Some(
                "MCP server for generating Loco framework model code from specifications"
                    .to_string(),
            ),
        })
    }
}
