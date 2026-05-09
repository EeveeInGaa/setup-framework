use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use console::style;

use crate::config::ReactSetupConfig;

pub fn install_dependencies(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Installing dependencies...", style("•").blue());

    let status = Command::new("npm")
        .arg("install")
        .current_dir(&config.project_path)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to run npm install. Is npm installed?")?;

    if !status.success() {
        bail!("npm install failed");
    }

    Ok(())
}