use std::path::Path;
use crate::download::dl_file;

/// Essentially just a wrapper over the general `dl_file` method.
pub async fn dl_mod(url: &str, path: Box<&Path>) -> Result<(), Box<dyn std::error::Error>> {
    dl_file(url, path).await?;

    Ok(())
}
