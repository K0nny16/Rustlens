use clap::{Parser, Subcommand};

use crate::{
    db::{connection::test_connection, errors::ConfigError},
    services::analysis::analyze_logs,
};

#[derive(Parser, Debug)]
#[command(
    name = "rustlens",
    version,
    about = "A tool for analyzing database tables"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Analyze {
        #[arg(short, long)]
        config: String,
    },

    TestConnection {
        #[arg(short, long)]
        config: String,
    },
}

pub fn run(command: Commands) -> Result<(), ConfigError> {
    match command {
        Commands::Analyze { config } => {
            analyze_logs(config)?;
            Ok(())
        }
        Commands::TestConnection { config } => {
            test_connection(config)?;
            Ok(())
        }
    }
}
