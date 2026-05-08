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

    if config.use_folders {
        steps::folders::create_folders(config)?;
    }

    if config.use_basic_components {
        steps::components::create_basic_components(config)?;
    }

    steps::vite_cleanup::cleanup_vite_template(config)?;

    steps::router::setup_router(config)?;

    if config.use_tailwind {
        steps::tailwind::setup_tailwind(config)?;
    }

    println!(
        "{} {}",
        style("✔").green(),
        style("Project created successfully").bold()
    );

    println!();
    println!("{}", style("Next steps will add Tailwind, layout and tooling.").dim());

    Ok(())
}