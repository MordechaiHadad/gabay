use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct BackupFile {
    pub paths: HashMap<PathBuf, PathInfo>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PathInfo {
    pub hash: String,
    pub last_backup: Option<DateTime<Utc>>,
}
