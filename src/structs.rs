use std::{collections::HashMap, path::PathBuf};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::helpers::Metadata;

pub type BackupFile = HashMap<PathBuf, BackupEntry>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct BackupEntry {
    pub hash: String,
    pub last_backup: Option<DateTime<Utc>>,
    pub metadata: Metadata,
}
