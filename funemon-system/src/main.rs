mod cli;
mod db;
mod mcp;
mod tui;

use clap::Parser;
use cli::{Cli, run_cli};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    run_cli(cli).await?;

    Ok(())
}