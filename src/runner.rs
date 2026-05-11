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

    steps::aliases::setup_aliases(config)?;

    steps::biome::setup_biome(config)?;

    steps::readme::setup_readme(config)?;

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

    println!(
        "  {} {} {} {}",
        style("activate").cyan(),
        style("Actions on save (all Biome Options)").bold(),
        "in Webstorm Settings",
        style("(Biome runs all files initially)").dim()
    );

    println!(
        "  {} {}",
        style("adjust").cyan(),
        "README.md",
    );

    println!(
        "  {} {}",
        style("adjust").cyan(),
        "Design Tokens",
    );

    println!(
        "  {} {}",
        style("add and adjust").cyan(),
        "Components and Pages as needed",
    );

    println!(
        "  {} {}",
        style("install").cyan(),
        "further packages",
    );

    Ok(())
}