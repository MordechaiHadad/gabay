use eyre::Result;
use tokio::fs;
use tracing::info;

use crate::helpers::dirs::get_backup_file;

pub async fn start() -> Result<()> {
    info!("Starting reset process");
    let path = get_backup_file()?;
    fs::remove_file(path).await?;

    info!("Backup file removed");
    Ok(())
}
