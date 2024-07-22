use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub type BackupFile = HashMap<PathBuf, BackupEntry>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BackupEntry {
    pub hash: String,
    pub last_backup: Option<DateTime<Utc>>,
}
