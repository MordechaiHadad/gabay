mod cli;
mod handlers;
mod helpers;
mod structs;

use std::process::exit;

use eyre::Result;
use tracing::{error, info, Level};
use tracing_appender::rolling::{RollingFileAppender, Rotation};
use tracing_subscriber::{fmt::writer::MakeWriterExt, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<()> {
    let file_appender = RollingFileAppender::new(Rotation::DAILY, "log", "gabay.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    let combined_writer = non_blocking.and(std::io::stdout);

    let subscriber = FmtSubscriber::builder()
        .with_target(false)
        .with_writer(combined_writer)
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber)?;

    if let Err(error) = run().await {
        error!("Error: {error}");
        exit(1);
    }

    Ok(())
}

async fn run() -> Result<()> {
    cli::start().await?;
    Ok(())
}
