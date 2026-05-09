use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use console::style;

use crate::config::ReactSetupConfig;

pub fn setup_git(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Initializing Git repository...", style("•").blue());

    run_git_command(
        config,
        &["init", "-b", "main"],
        "Failed to initialize git repository",
    )?;

    run_git_command(
        config,
        &["add", "."],
        "Failed to add files to git",
    )?;

    run_git_command(
        config,
        &["commit", "-m", "initial commit"],
        "Failed to create initial git commit",
    )?;

    Ok(())
}

fn run_git_command(
    config: &ReactSetupConfig,
    args: &[&str],
    error_message: &str,
) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .current_dir(&config.project_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .with_context(|| error_message.to_string())?;

    if !status.success() {
        bail!(error_message.to_string());
    }

    Ok(())
}