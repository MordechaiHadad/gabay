pub mod dirs;
pub mod fs;

use std::path::Path;

use async_recursion::async_recursion;
use eyre::Result;
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

    pub async fn hash_file<P: AsRef<Path>>(&mut self, path: P) -> Result<String> {
        let file = File::open(path).await?;
        let mut reader = BufReader::new(file);

        let mut buffer = [0; 1024];
        loop {
            let bytes_read = reader.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            self.hasher.update(&buffer[..bytes_read]);
        }

        let result = self.hasher.finalize_reset();
        Ok(format!("{:x}", result))
    }

    #[async_recursion(?Send)]
    pub async fn hash_dir<P: AsRef<Path>>(&mut self, path: P) -> Result<String> {
        let mut entries = read_dir(path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_dir() {
                self.hash_dir(path).await?;
            } else {
                self.hash_file(path).await?;
            }
        }

        let result = self.hasher.finalize_reset();
        Ok(format!("{:x}", result))
    }
}
