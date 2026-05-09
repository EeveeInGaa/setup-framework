use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use console::style;

use crate::config::ReactSetupConfig;

pub fn finalize_project(config: &ReactSetupConfig) -> Result<()> {
    println!(
        "{} Running Biome formatting and linting...",
        style("•").blue()
    );

    let status = Command::new("npx")
        .args(["biome", "check", "--write", "."])
        .current_dir(&config.project_path)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run Biome")?;

    if !status.success() {
        bail!("Biome check failed");
    }

    Ok(())
}