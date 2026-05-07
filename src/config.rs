#[derive(Debug, Clone)]
pub struct ReactSetupConfig {
    pub app_name: String,
    pub setup_mode: SetupMode,
}

#[derive(Debug, Clone, Copy)]
pub enum SetupMode {
    Simple,
    Advanced,
}