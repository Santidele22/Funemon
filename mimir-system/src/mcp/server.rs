use super::tools::MemoryTools;
use rmcp::{transport::stdio, ServiceExt};

pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let handler = MemoryTools::new();

    let service = handler.serve(stdio()).await?;
    service.waiting().await?;
    Ok(())
}
