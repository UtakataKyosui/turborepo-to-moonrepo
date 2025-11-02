mod error;
mod generator;
mod models;
mod server;
mod validation;

use rmcp::ServiceExt;
use server::LocoModelServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = LocoModelServer::new();

    // Serve over stdio (standard MCP transport)
    let service = server
        .serve((tokio::io::stdin(), tokio::io::stdout()))
        .await?;

    // Wait until server terminates
    service.waiting().await?;

    Ok(())
}
