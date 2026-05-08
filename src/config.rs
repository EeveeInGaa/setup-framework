use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct ReactSetupConfig {
    pub app_name: String,
    pub setup_mode: SetupMode,
    pub project_path: PathBuf,

    pub language: Language,
    pub package_manager: PackageManager,

    pub use_git: bool,

    pub linting: Vec<LintingTool>,
    pub formatting: FormattingTool,

    pub use_tailwind: bool,
    pub use_layout: bool,
    pub use_global_styles: bool,
    pub use_folders: bool,
    pub use_basic_components: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum SetupMode {
    Simple,
    Advanced,
}

#[derive(Debug, Clone, Copy)]
pub enum Language {
    TypeScript,
    JavaScript,
}

#[derive(Debug, Clone, Copy)]
pub enum PackageManager {
    Npm,
    Pnpm,
    Yarn,
    Bun,
}

#[derive(Debug, Clone, Copy)]
pub enum LintingTool {
    Biome,
    Stylelint,
    Eslint,
}

#[derive(Debug, Clone, Copy)]
pub enum FormattingTool {
    Prettier,
    Biome,
}