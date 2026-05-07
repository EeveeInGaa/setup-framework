use anyhow::Result;
use inquire::{Select, Text};

use crate::config::{ReactSetupConfig, SetupMode};

pub fn collect_react_setup_config(app_name: Option<String>) -> Result<ReactSetupConfig> {
    let app_name = match app_name {
        Some(name) => name,
        None => Text::new("Name of the project:").prompt()?,
    };

    let setup_mode = Select::new(
        "Choose setup mode:",
        vec![
            "Simple setup (quick and easy)",
            "Advanced setup (full control)",
        ],
    )
        .with_starting_cursor(0)
        .prompt()?;

    let setup_mode = match setup_mode {
        "Simple setup (quick and easy)" => SetupMode::Simple,
        "Advanced setup (full control)" => SetupMode::Advanced,
        _ => SetupMode::Simple,
    };

    Ok(ReactSetupConfig {
        app_name,
        setup_mode,
    })
}