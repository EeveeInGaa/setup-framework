mod cli;
mod config;
mod prompts;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use console::style;
use prompts::collect_react_setup_config;

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::React { app_name } => {
            let config = collect_react_setup_config(app_name)?;

            println!(
                "{} {}",
                style("✔").green(),
                style(format!("React setup prepared for '{}'", config.app_name)).bold()
            );

            println!("Mode: {:?}", config.setup_mode);
        }
    }

    Ok(())
}