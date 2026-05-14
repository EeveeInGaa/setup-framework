use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use console::style;

use crate::config::{ReactSetupConfig, PackageManager};

pub fn create_vite_project(config: &ReactSetupConfig) -> Result<()> {
    println!(
        "{} Creating Vite React TypeScript template...",
        style("•").blue()
    );

    let mut command = match config.package_manager {
        PackageManager::Pnpm => {
            let mut command = Command::new("pnpm");
            command.args([
                "create",
                "vite",
                &config.app_name,
                "--template",
                "react-ts",
                "--no-install",
            ]);
            command
        }
        PackageManager::Npm => {
            let mut command = Command::new("npx");
            command.args([
                "create-vite@latest",
                &config.app_name,
                "--template",
                "react-ts",
                "--no-install",
            ]);
            command
        }
        PackageManager::Yarn | PackageManager::Bun => {
            unimplemented!("This package manager is not supported yet");
        }
    };

    let output = command
        .stdin(Stdio::null())
        .output()
        .context("Failed to create Vite project")?;

    if !output.status.success() {
        bail!("Vite project creation failed");
    }

    Ok(())
}