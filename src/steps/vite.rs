use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use console::style;

use crate::config::ReactSetupConfig;

pub fn create_vite_project(config: &ReactSetupConfig) -> Result<()> {
    println!(
        "{} Creating Vite React TypeScript template...",
        style("•").blue()
    );

    let output = Command::new("npx")
        .args([
            "create-vite@latest",
            &config.app_name,
            "--template",
            "react-ts",
            "--no-install",
        ])
        .stdin(Stdio::null())
        .output()
        .context("Failed to run npx. Is Node.js/npm installed?")?;

    if !output.status.success() {
        bail!("Vite project creation failed");
    }

    Ok(())
}