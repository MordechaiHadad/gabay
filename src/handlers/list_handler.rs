use chrono::{format, DateTime, Utc};
use eyre::Result;

use crate::structs::BackupEntry;

const TABLE_WIDTH: usize = 62;

pub async fn start() -> Result<()> {
    let backup_file = crate::helpers::dirs::get_backup_file()?;
    let content = tokio::fs::read_to_string(&backup_file).await?;

    let backup: crate::structs::BackupFile = serde_json::from_str(&content)?;
    let total_paths = backup.len();
    let last_backup = backup
        .iter()
        .map(|p| p.1.last_backup)
        .max()
        .unwrap_or_default();

    let header = format!("Backup Paths Overview");
    println!(
        "╔═══ {header} {}╗", "═".repeat(TABLE_WIDTH)
    );
    let total_paths_string = format!("Total Paths: {}", total_paths);
    println!(
        "║ {total_paths_string}{}║",
        " ".repeat(TABLE_WIDTH + header.len() - total_paths_string.len() + 4)
    );
    let last_backup_string = format!("Last Backup: {}", format_last_backup(last_backup));
    println!(
        "║ {last_backup_string}{}║",
        " ".repeat(TABLE_WIDTH + header.len() - last_backup_string.len() + 4)
    );
    println!(
        "╠{}╣", "═".repeat(TABLE_WIDTH + header.len() + 5)
    );

    for (index, (path, entry)) in backup.iter().enumerate() {
        let truncated_path = if path.to_str().unwrap().len() > 60 {
            format!("{}...", &path.to_str().unwrap()[..58])
        } else {
            path.to_str().unwrap().to_string()
        };

        println!("║ {}. {:<77} ║", index + 1, truncated_path);
        println!(
            "║    └─ Last Backup: {:<67} ║",
            format_last_backup(entry.last_backup)
        );

        if index < backup.len() - 1 {
            println!("╟───────────────────────────────────────────────────────────────────────────────────────╢");
        }
    }

    println!(
        "╚═══════════════════════════════════════════════════════════════════════════════════════╝"
    );

    Ok(())
}

fn format_last_backup(last_backup: Option<DateTime<Utc>>) -> String {
    match last_backup {
        Some(date_time) => date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "None".to_string(),
    }
}
