use eyre::Result;
use indicatif::{ProgressBar, ProgressStyle};
use std::path::PathBuf;
use tokio::fs;

use crate::{
    helpers::{dirs::get_backup_file, get_metadata, Hasher},
    structs::BackupFile,
};

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
        hasher.hash_file(&path, &pb, true).await?
    } else {
        hasher.hash_dir(&path, &pb).await?
    };

    pb.finish_with_message("Hashing complete");

    let backup_file = get_backup_file()?;

    let content = if backup_file.exists() {
        fs::read_to_string(&backup_file).await?
    } else {
        return Err(eyre::eyre!("Backup file does not exist"));
    };

    let backup: BackupFile = serde_json::from_str(&content)?;

    if !backup.contains_key(&path) {
        return Err(eyre::eyre!("Path is not inside backup file"));
    }

    let old_metadata = backup.get(&path).unwrap().metadata.clone();
    let old_hash = backup.get(&path).unwrap().hash.clone();

    println!(
        "Are hashes equal: {}, Are metadata equal: {}",
        old_hash == hash,
        old_metadata == metadata
    );
    Ok(())
}
