use anyhow::Result;
use inquire::{Select, Text};

use crate::config::{
    FormattingTool,
    Language,
    LintingTool,
    PackageManager,
    ReactSetupConfig,
    SetupMode,
};

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

    match setup_mode {
        SetupMode::Simple => Ok(create_simple_config(app_name)),
        SetupMode::Advanced => {
            println!();
            println!("Advanced setup is still work in progress.");

            std::process::exit(0);
        }
    }
}

fn create_simple_config(app_name: String) -> ReactSetupConfig {
    ReactSetupConfig {
        app_name,
        setup_mode: SetupMode::Simple,

        language: Language::TypeScript,
        package_manager: PackageManager::Npm,

        use_git: true,

        linting: vec![
            LintingTool::Biome,
            LintingTool::Stylelint,
        ],

        formatting: FormattingTool::Biome,

        use_tailwind: true,
        use_layout: true,
        use_global_styles: true,
        use_folders: true,
        use_basic_components: true,
    }
}