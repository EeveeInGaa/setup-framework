use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "setup")]
#[command(about = "A small CLI for setting up frontend projects")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    React {
        app_name: Option<String>,
    },
}