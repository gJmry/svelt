use create::*;
use clap::{CommandFactory, Parser, Subcommand};
use crate::models::commands::Commands;
use crate::models::cli::Cli;
use crate::controller::create;
use crate::controller::npm::install;

pub fn run(args: Vec<String>) {
    let cli = Cli::parse();


    match cli.command {
        Some(Commands::Init { name, ui_toolkit }) => {
            project_creation::main(name, ui_toolkit);
        }
        Some(Commands::Version) => {
            println!("Version 1.0.0");
        }
        Some(Commands::Install {package, flag}) => {
            install::main(package, flag);
        }
        Some(Commands::Help) | None => {
            Cli::command().print_help().unwrap();
        }
        _ => {Cli::command().print_help().unwrap();}
    }
}