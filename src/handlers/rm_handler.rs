use eyre::Result;
use std::path::PathBuf;
use tracing::info;

use crate::{helpers::dirs::get_backup_file, structs::BackupFile};

pub async fn start(path: PathBuf) -> Result<()> {
    info!("Removing path from backup file");

    if !path.is_absolute() {
        return Err(eyre::eyre!("Path must be absolute"));
    }

    let backup_file = get_backup_file()?;
    if !backup_file.exists() {
        return Err(eyre::eyre!("Backup file does not exist"));
    }

    let backup = tokio::fs::read_to_string(&backup_file).await?;
    let mut backup: BackupFile = serde_json::from_str(&backup)?;
    if !backup.contains_key(&path) {
        return Err(eyre::eyre!("Path is not inside backup file"));
    }

    backup.remove(&path);
    let content = serde_json::to_string(&backup)?;
    tokio::fs::write(&backup_file, content).await?;
    info!("Path removed from backup file");
    Ok(())
}
