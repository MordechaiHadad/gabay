use std::path::PathBuf;

use clap::Parser;
use eyre::Result;
use tracing::info;

use crate::handlers::{add_handler, backup_handler, list_handler, reset_handler, rm_handler};

#[derive(Debug, Parser)]
#[command(version)]
enum Cli {
    Add {
        path: PathBuf,
    },

    #[clap(alias = "rm")]
    Remove {
        path: PathBuf,
    },

    Backup {
        destination: PathBuf,
    },

    #[clap(alias = "erase")]
    Reset,

    #[clap(alias = "ls")]
    List,
}

pub async fn start() -> Result<()> {
    let args = Cli::parse();
    match args {
        Cli::Add { path } => {
            add_handler::start(path).await?;
        }
        Cli::Remove { path } => {
            rm_handler::start(path).await?;
        }
        Cli::Backup { destination } => {
            backup_handler::start(destination).await?;
        }
        Cli::Reset => {
            reset_handler::start().await?;
        }
        Cli::List => list_handler::start().await?,

    }
    Ok(())
}
