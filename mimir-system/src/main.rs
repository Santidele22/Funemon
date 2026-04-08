mod cli;
mod db;
mod mcp;
mod reflection;

use clap::Parser;
use cli::{Cli, run_cli};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    if std::env::args().len() > 1 {
        return run_cli(cli);
    }

    db::init_database()?;
    mcp::run_server().await?;

    Ok(())
}
