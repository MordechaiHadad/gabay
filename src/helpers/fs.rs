use std::path::Path;
use tokio::fs;
use eyre::Result;

use async_recursion::async_recursion;

#[async_recursion(?Send)]
pub async fn copy_dir(
    from: impl AsRef<Path> + 'static,
    to: impl AsRef<Path> + 'static,
) -> Result<()> {
    let original_path = from.as_ref().to_owned();
    let destination = to.as_ref().to_owned();

    fs::create_dir(&destination).await?;

    let mut entries = fs::read_dir(original_path).await?;

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();

        if path.is_dir() {
            let new_dest = destination.join(path.file_name().unwrap());
            copy_dir(path, new_dest).await?;
        } else {
            let new_dest = destination.join(path.file_name().unwrap());
            fs::copy(path, new_dest).await?;
        }
    }

    Ok(())
}