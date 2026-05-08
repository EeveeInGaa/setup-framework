use std::fs;

use anyhow::{Context, Result};
use console::style;

use crate::config::ReactSetupConfig;

pub fn create_folders(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Creating project folders...", style("•").blue());

    let folders = [
        "src/components",
        "src/utils",
        "src/core",
        "src/pages",
        "src/pages/general",
        "src/pages/feature",
    ];

    for folder in folders {
        let path = config.project_path.join(folder);

        fs::create_dir_all(&path)
            .with_context(|| format!("Failed to create folder '{}'", path.display()))?;
    }

    Ok(())
}