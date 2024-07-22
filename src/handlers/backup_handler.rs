use chrono::Utc;
use eyre::Result;
use std::path::PathBuf;
use tracing::{info, warn};

use crate::{
    helpers::{fs::replace_file, Hasher},
    structs::BackupFile,
};

/// Starts the backup process.
///
/// This function reads the backup file, checks for changes in the files/directories listed in the backup,
/// and performs the backup if necessary. It updates the backup file with the latest backup information.
///
/// # Arguments
///
/// * `dest` - The destination directory where the backup files will be stored.
///
/// # Returns
///
/// Returns `Ok(())` if the backup process completes successfully, or an `eyre::Report` if an error occurs.
///
/// # Panics
///
/// This function will panic if it encounters an unrecoverable error during the backup process.
///
/// # Errors
///
/// This function will return an error if it fails to read the backup file, fails to parse the backup file,
/// encounters an error while hashing files/directories, fails to replace a file during backup, fails to copy
/// a directory during backup, or fails to write the updated backup file.
pub async fn start(dest: PathBuf) -> Result<()> {
    info!("Starting backup process");
    let backup_file = crate::helpers::dirs::get_backup_file()?;
    let backup = tokio::fs::read_to_string(&backup_file).await?;
    let mut backup: BackupFile = serde_json::from_str(&backup)?;

    let mut hasher = Hasher::new();
    let mut updates: Vec<(PathBuf, chrono::DateTime<Utc>, String)> = Vec::new();

    let now = chrono::Utc::now();

    for (path, info) in backup.iter() {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        let is_file = path.is_file();
        let hash = if is_file {
            hasher.hash_file(path).await?
        } else {
            hasher.hash_dir(path).await?
        };

        if hash == info.hash && info.last_backup.is_some() {
            continue;
        }

        info!("{} has changed, backing up", path.display());
        let backup_dist = dest.join(file_name);

        if is_file {
            replace_file(path, backup_dist).await?;
        } else {
            crate::helpers::fs::copy_dir(path.clone(), backup_dist).await?;
        }
        updates.push((path.clone(), now, hash));
    }

    if updates.is_empty() {
        warn!("There is nothing to backup");
        return Ok(());
    }

    for (path, now, hash) in updates {
        if let Some(info) = backup.get_mut(&path) {
            info.last_backup = Some(now);
            info.hash = hash;
        }
    }

    let content = serde_json::to_string(&backup)?;
    tokio::fs::write(&backup_file, content).await?;

    info!("Backup process finished");
    Ok(())
}
