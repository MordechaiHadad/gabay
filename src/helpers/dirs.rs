use eyre::Result;
use std::{fs, path::PathBuf};

pub fn get_backup_dir() -> PathBuf {
    let dir = dirs::data_local_dir().unwrap().join("gabay");

    dir.into()
}

pub fn get_backup_file() -> Result<PathBuf> {
    let base = get_backup_dir();
    fs::create_dir_all(&base)?;
    Ok(base.join("backup.json"))
}
