use clap::{Parser, Subcommand};
use crate::{ config, errors::Result};

#[derive(Parser)]
#[clap(name = "jvm")]
#[clap(about = "Java Version", version = "0.1")]
pub struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    #[clap(about = "List all available Java versions")]
    List,
    #[clap(about = "Switch to a specific Java version")]
    Use {
        #[clap(help = "Java version to switch to")]
        version: String,
    }
}

impl Cli {
    pub async fn run(&self) -> Result<()> {
        match &self.command {
            Commands::List => {
                config::list_java_versions()?;
            }
            Commands::Use { version } => {
                config::switch_java_version(version)?;
                println!("Switched to Java version {}", version);
            }
        }
        Ok(())
    }
}