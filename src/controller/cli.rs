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
        Some(command) => match command {
            Commands::Init { name } => {
                project_creation::main(name);
            }
            Commands::Version => {
                println!("Version 1.0.0");
            }
           Commands::Install { package, flag } => {
                install::main(package, flag);
            }
            Commands::Run { env } => {
                run::main(env);
            }
            Commands::Start => {
                start::main();
            }
            Commands::Update { package } => {
                update::main(package);
            }
            Commands::Uninstall { package } => {
                uninstall::main(package);
            }
            Commands::Create { schematic, name, args } => {
                create::main(schematic, name, args);
            }
            Commands::Dev => {
                run::dev();
            }
            Commands::Build => {
                run::build();
            }
            Commands::Lint => {}
            Commands::Help => {
                Cli::command().print_help().unwrap();
            },
        }
        None => {
            Cli::command().print_help().unwrap();
        },

    }
}
