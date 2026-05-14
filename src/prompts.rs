use anyhow::Result;
use inquire::{Select, Text};
use std::path::PathBuf;

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

    if matches!(setup_mode, SetupMode::Advanced) {
        println!();
        println!("Advanced setup is still work in progress.");

        std::process::exit(0);
    }

    let package_manager = Select::new(
        "Choose package manager:",
        vec![
            "pnpm (recommended)",
            "npm",
        ],
    )
        .with_starting_cursor(0)
        .prompt()?;

    let package_manager = match package_manager {
        "pnpm (recommended)" => PackageManager::Pnpm,
        "npm" => PackageManager::Npm,
        _ => PackageManager::Pnpm,
    };

    Ok(create_simple_config(app_name, package_manager))
}

fn create_simple_config(
    app_name: String,
    package_manager: PackageManager,
) -> ReactSetupConfig {
    let project_path = PathBuf::from(&app_name);

    ReactSetupConfig {
        app_name,
        setup_mode: SetupMode::Simple,
        project_path,

        language: Language::TypeScript,
        package_manager,

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