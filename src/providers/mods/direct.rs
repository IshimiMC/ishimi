use crate::providers::dl_file;
use std::path::Path;

/// Essentially just a wrapper over the general `dl_file` method.
pub async fn dl_mod(url: &str, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    dl_file(url, path).await?;

    Ok(())
}
