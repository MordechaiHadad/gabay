use std::path::PathBuf;

use eyre::Result;
use indicatif::{ProgressBar, ProgressStyle};
use tokio::fs::{self};
use tracing::info;
use tracing_subscriber::fmt::format;

use crate::{
    helpers::{dirs::get_backup_file, get_metadata, Hasher},
    structs::{BackupEntry, BackupFile},
};

/// Adds a path to the backup file.
///
/// # Errors
///
/// This function will return an error if .
pub async fn start(path: PathBuf) -> Result<()> {
    if !path.is_absolute() {
        return Err(eyre::eyre!("Path must be absolute"));
    }

    let mut hasher = Hasher::new();

    let metadata = get_metadata(&path).await?;

    let pb = ProgressBar::new(metadata.size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg}\n{wide_bar} {bytes}/{total_bytes} ({eta})")?
            .progress_chars("#>-"),
    );
    pb.set_message(format!("Hashing {}", path.display()));

    let hash = if path.is_file() {
        hasher.hash_file(&path, &pb).await?
    } else {
        hasher.hash_dir(&path, &pb).await?
    };

    pb.finish_with_message("Hashing complete");

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

    if backup.contains_key(&path) {
        return Err(eyre::eyre!("Path is already inside backup file"));
    }

    backup.insert(
        path,
        BackupEntry {
            hash,
            last_backup: None,
            metadata,
        },
    );

    let content = serde_json::to_string(&backup)?;
    fs::write(backup_file, content).await?;
    info!("Path added to backup file");
    Ok(())
}
