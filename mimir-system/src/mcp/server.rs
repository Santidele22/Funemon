use super::tools::MemoryTools;
use rmcp::{service::serve_server, transport::stdio};
pub async fn run_server() -> Result<(), Box<dyn std::error::Error>> {
    let handler = MemoryTools::new();

    serve_server(handler, stdio()).await?;
    Ok(())
}

