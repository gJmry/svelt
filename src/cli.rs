mod project_creation;
mod help;
mod version_check;

use clap::{CommandFactory, Parser, Subcommand};
use crate::commands::Commands;

#[derive(Parser)]
#[command(author = "Jérémy Girard", version = "0.1", about = "A CLI made in Rust for Svelte", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

pub fn run(args: Vec<String>) {
    let cli = Cli::parse();


    match cli.command {
        Some(Commands::Init { name, ui_toolkit }) => {
            project_creation::main(name, ui_toolkit);
        }
        Some(Commands::Version) => {
            println!("Version 1.0.0");
        }
        Some(Commands::Help) | None => {
            Cli::command().print_help().unwrap();
        }
    }
}