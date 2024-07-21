use std::path::PathBuf;

use eyre::Result;
use tokio::{
    fs::{self, File},
    io::{AsyncReadExt, AsyncWriteExt},
};
use tracing::{debug, info};

use crate::{
    helpers::{dirs::get_backup_file, Hasher},
    structs::{BackupFile, PathInfo},
};

pub async fn start(path: PathBuf) -> Result<()> {
    if !path.is_absolute() {
        return Err(eyre::eyre!("Path must be absolute"));
    }

    let mut hasher = Hasher::new();

    let hash = if path.is_file() {
        hasher.hash_file(&path).await?
    } else {
        hasher.hash_dir(&path).await?
    };

    let backup_file = get_backup_file()?;

    let content = if backup_file.exists() {
        Some(fs::read_to_string(&backup_file).await?)
    } else {
        fs::write(&backup_file, "").await?;
        None
    };

    let mut backup: BackupFile = match content {
        Some(content) => serde_json::from_str(&content)?,
        None => Default::default(),
    };

    if backup.paths.contains_key(&path) {
        return Err(eyre::eyre!("Path is already inside backup file"));
    }

    backup.paths.insert(
        path,
        PathInfo {
            hash,
            last_backup: None,
        },
    );

    let content = serde_json::to_string(&backup)?;
    fs::write(backup_file, content).await?;
    info!("Path added to backup file");
    Ok(())
}
