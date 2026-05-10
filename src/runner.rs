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

    steps::biome::setup_biome(config)?;

    steps::install::install_dependencies(config)?;

    steps::finalize::finalize_project(config)?;

    steps::git::setup_git(config)?;

    println!();

    println!(
        "{} {}",
        style("✔").green(),
        style("Project created successfully").bold()
    );

    println!();

    println!("{}", style("Next steps:").bold());

    println!(
        "  {} {}",
        style("cd").cyan(),
        config.app_name
    );

    println!(
        "  {}",
        style("ws").cyan(),
    );

    println!(
        "  {} {} {}",
        style("npm").cyan(),
        "run dev",
        style("(or run directly in IDE)").dim()
    );

    Ok(())
}