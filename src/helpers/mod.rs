pub mod dirs;
pub mod fs;

use std::path::Path;

use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use eyre::Result;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use tokio::{
    fs::{read_dir, File},
    io::{AsyncReadExt, BufReader},
};

pub struct Hasher {
    hasher: Sha256,
}

impl Hasher {
    pub fn new() -> Self {
        Self {
            hasher: Sha256::new(),
        }
    }

    pub async fn hash_file<P: AsRef<Path>>(
        &mut self,
        path: P,
        pb: &ProgressBar,
        finalize: bool,
    ) -> Result<String> {
        let file = File::open(path).await?;
        let mut reader = BufReader::with_capacity(8192 * 1024, file);

        let mut buffer = vec![0; 8192 * 1024];

        loop {
            let bytes_read = reader.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            self.hasher.update(&buffer[..bytes_read]);
            pb.inc(bytes_read as u64);
        }

        if finalize {
            let result = self.hasher.finalize_reset();
            return Ok(format!("{:x}", result));
        }

        Ok("".to_string())
    }

    #[async_recursion(?Send)]
    pub async fn hash_dir<P: AsRef<Path>>(&mut self, path: P, pb: &ProgressBar) -> Result<String> {
        let mut entries = read_dir(path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                self.hash_dir(path, pb).await?;
            } else {
                self.hash_file(path, pb, false).await?;
            }
        }

        let result = self.hasher.finalize_reset();
        Ok(format!("{:x}", result))
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Metadata {
    pub size: u64,
    pub last_modified: DateTime<Utc>,
}

pub async fn get_metadata(path: &Path) -> Result<Metadata> {
    let metadata = path.metadata()?;
    let size = get_path_size(path).await?;

    Ok(Metadata {
        size,
        last_modified: metadata.modified()?.into(),
    })
}

#[async_recursion(?Send)]
pub async fn get_path_size(path: &Path) -> Result<u64> {
    if path.is_dir() {
        let mut size = 0;
        let mut entries = read_dir(path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            size += get_path_size(&path).await?;
        }
        return Ok(size);
    } else {
        return Ok(path.metadata()?.len());
    }
}
