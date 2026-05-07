use anyhow::Result;
use console::style;

use crate::config::ReactSetupConfig;
use crate::steps;

pub fn run_react_setup(config: &ReactSetupConfig) -> Result<()> {
    println!(
        "{} {}",
        style("➜").cyan(),
        style(format!("Creating React project '{}'", config.app_name)).bold()
    );

    steps::vite::create_vite_project(config)?;

    println!(
        "{} {}",
        style("✔").green(),
        style("Project created successfully").bold()
    );

    println!();
    println!("{}", style("Next steps will add folders, Tailwind, layout and tooling.").dim());

    Ok(())
}