mod db;
use crate::db::models;
mod mcp;
mod reflection;

use db::init_database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    init_database()?;
    println!("✅ Base de datos inicializada");

    println!("🚀 Iniciando servidor MCP...");
    mcp::run_server().await?;

    Ok(())
}
