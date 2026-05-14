use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use console::style;

use crate::config::{PackageManager, ReactSetupConfig};

pub fn install_dependencies(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Installing dependencies...", style("•").blue());

    let install_command = match config.package_manager {
        PackageManager::Pnpm => "pnpm",
        PackageManager::Npm => "npm",
        PackageManager::Yarn | PackageManager::Bun => {
            unimplemented!("This package manager is not supported yet");
        }
    };

    let status = Command::new(install_command)
        .arg("install")
        .current_dir(&config.project_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status()
        .context("Failed to install dependencies")?;

    if !status.success() {
        bail!("Dependency installation failed");
    }

    Ok(())
}