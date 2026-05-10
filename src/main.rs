mod cli;
mod config;
mod prompts;
mod runner;
mod steps;
mod utils;

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
            /* println!("{:#?}", config); shows config structure */
            runner::run_react_setup(&config)?;
        }
    }

    Ok(())
}