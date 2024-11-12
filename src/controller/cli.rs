use crate::controller::create::*;
use crate::controller::init;
use crate::controller::npm::*;
use crate::models::cli::Cli;
use crate::models::commands::Commands;
use clap::{CommandFactory, Parser};
use init::*;

pub fn run() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { name }) => {
            project_creation::main(name);
        }
        Some(Commands::Version) => {
            println!("Version 1.0.0");
        }
        Some(Commands::Install { package, flag }) => {
            install::main(package, flag);
        }
        Some(Commands::Run { env }) => {
            run::main(env);
        }
        Some(Commands::Start) => {
            start::main();
        }
        Some(Commands::Update { package }) => {
            update::main(package);
        }
        Some(Commands::Uninstall { package }) => {
            uninstall::main(package);
        }
        Some(Commands::Create { schematic, name }) => {
            create::main(schematic, name);
        }
        Some(Commands::Dev) => {
            run::dev();
        }
        Some(Commands::Build) => {
            run::build();
        }
        Some(Commands::Lint) => {}
        Some(Commands::Help) | None => {
            Cli::command().print_help().unwrap();
        }
    }
}
