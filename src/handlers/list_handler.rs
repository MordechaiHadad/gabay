use chrono::{DateTime, Utc};
use eyre::Result;

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

    let header = "Backup Paths Overview".to_string();
    let header_length = header.len();
    println!("╔═══ {header} {}╗", "═".repeat(TABLE_WIDTH));
    let total_paths_string = format!("Total Paths: {}", total_paths);
    println!("║ {total_paths_string:<87}║");
    let last_backup_string = format!("Last Backup: {}", format_last_backup(last_backup));
    println!("║ {last_backup_string:<87}║",);
    println!("╠{}╣", "═".repeat(TABLE_WIDTH + header_length + 5));

    for (index, (path, entry)) in backup.iter().enumerate() {
        let truncated_path = if path.to_str().unwrap().len() > 60 {
            format!("{}...", &path.to_str().unwrap()[..58])
        } else {
            path.to_str().unwrap().to_string()
        };

        println!("║ {}. {truncated_path:<84}║", index + 1);
        println!("║    └─ Hash: {:<74} ║", entry.hash);
        println!(
            "║    └─ Size: {:<74} ║",
            humanize_bytes(entry.metadata.size)
        );
        println!(
            "║    └─ Last Modified: {:<65} ║",
            entry.metadata.last_modified.format("%Y-%m-%d %H:%M:%S")
        );
        println!(
            "║    └─ Last Backup: {:<67} ║",
            format_last_backup(entry.last_backup)
        );

        if index < backup.len() - 1 {
            println!("╟{}╢", "─".repeat(TABLE_WIDTH + header_length + 5));
        }
    }

    println!("╚{}╝", "═".repeat(TABLE_WIDTH + header_length + 5));

    Ok(())
}

fn format_last_backup(last_backup: Option<DateTime<Utc>>) -> String {
    match last_backup {
        Some(date_time) => date_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        None => "None".to_string(),
    }
}

fn humanize_bytes(bytes: u64) -> String {
    let sizes = ["Bytes", "KB", "MB", "GB", "TB"];
    if bytes == 0 {
        return "0 Bytes".to_string();
    }
    let i = (bytes as f64).log(1024.0).floor() as i32;
    if i == 0 {
        return format!("{} {}", bytes, sizes[i as usize]);
    }
    let size = (bytes as f64) / 1024.0_f64.powi(i);
    format!("{:.3} {}", size, sizes[i as usize])
}
