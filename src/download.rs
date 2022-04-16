mod loader_download;
pub mod mod_download;

use std::fs::File;
use std::io::Write;
use std::path::Path;

async fn dl_file(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;

    let mut file = File::create(&*path)?;

    let content = response.bytes().await?;
    file.write_all(&*content)?;

    Ok(())
}
