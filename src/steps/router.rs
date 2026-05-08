use anyhow::Result;
use console::style;

use crate::config::ReactSetupConfig;
use crate::utils::package_json::add_dependency;

pub fn setup_router(config: &ReactSetupConfig) -> Result<()> {
    println!("{} Adding React Router...", style("•").blue());

    add_dependency(
        &config.project_path,
        "react-router",
        "^7.6.0",
    )?;

    Ok(())
}