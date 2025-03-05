mod cli;
mod config;
mod errors;

use clap::Parser;
use cli::Cli;
use errors::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run().await?;
    Ok(())
}