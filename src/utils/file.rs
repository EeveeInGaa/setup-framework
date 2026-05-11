use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

pub fn remove_file_if_exists(path: PathBuf) -> Result<()> {
    if path.exists() {
        fs::remove_file(&path)
            .with_context(|| format!("Failed to remove file '{}'", path.display()))?;
    }

    Ok(())
}