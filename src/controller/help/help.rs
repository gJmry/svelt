use cliclack::{intro, outro, select};
use console::{style};
use std::io::{self, Write};

pub fn main() -> std::io::Result<()> {
    intro(style("Svelte-Cli commands").on_cyan().bright())?;

    loop {
        let category = select("Pick a page you want to see".to_string())
            .initial_value("Utils")
            .item("Utils", "Utility commands for Svelte", "")
            .item("Generation", "Project generation commands", "")
            .item("Testing", "Testing commands", "")
            .item("Deployment", "Deployment commands", "")
            .item("Docs", "Documentation commands", "")
            .interact()?;

        match category {
            "Utils" => {
                show_utils_commands()?;
            }
            "Generation" => {
                show_generation_commands()?;
            }
            "Testing" => {
                show_testing_commands()?;
            }
            "Deployment" => {
                show_deployment_commands()?;
            }
            "Docs" => {
                show_docs_commands()?;
            }
            _ => {
                outro("Unknown option! Please check the documentation.").expect("Error when prompting the outro");
                return Ok(());
            }
        }
        prompt_to_return()?;
    }
}

fn show_utils_commands() -> std::io::Result<()> {
    println!("{}", style("=== Utils Commands ===").cyan().bold());
    println!("{}", format!("{} : Creates a new Svelte project.", style("init <project_name> <ui_toolkit>").yellow().bold()));
    println!("{}", format!("{} : Installs project dependencies.", style("install").yellow().bold()));
    println!("{}", format!("{} : Updates project dependencies.", style("update").yellow().bold()));
    println!("{}", format!("{} : Cleans up temporary files.", style("clean").yellow().bold()));
    println!("{}", format!("{} : Displays the project version.", style("version").yellow().bold()));
    Ok(())
}

fn show_generation_commands() -> std::io::Result<()> {
    println!("{}", style("=== Generation Commands ===").cyan().bold());
    println!("{}", format!("{} : Compiles the project for production.", style("build").yellow().bold()));
    println!("{}", format!("{} : Starts the local development server.", style("dev").yellow().bold()));
    println!("{}", format!("{} : Starts the application in production mode.", style("start").yellow().bold()));
    println!("{}", format!("{} : Analyzes the size of the bundle.", style("analyze").yellow().bold()));
    println!("{}", format!("{} : Optimizes assets for production.", style("optimize").yellow().bold()));
    Ok(())
}

fn show_testing_commands() -> std::io::Result<()> {
    println!("{}", style("=== Testing Commands ===").cyan().bold());
    println!("{}", format!("{} : Runs unit tests.", style("test").yellow().bold()));
    println!("{}", format!("{} : Lints the project files.", style("lint").yellow().bold()));
    println!("{}", format!("{} : Formats the project code using Prettier.", style("format").yellow().bold()));
    Ok(())
}

fn show_deployment_commands() -> std::io::Result<()> {
    println!("{}", style("=== Deployment Commands ===").cyan().bold());
    println!("{}", format!("{} : Deploys the application.", style("deploy").yellow().bold()));
    println!("{}", format!("{} : Runs the continuous integration pipeline.", style("ci").yellow().bold()));
    Ok(())
}

fn show_docs_commands() -> std::io::Result<()> {
    println!("{}", style("=== Docs Commands ===").cyan().bold());
    println!("{}", format!("{} : Generates project documentation.", style("docs").yellow().bold()));
    println!("{}", format!("{} : Displays or generates the changelog.", style("changelog").yellow().bold()));
    println!("{}", format!("{} : Displays the version of the documentation.", style("version").yellow().bold()));
    Ok(())
}

fn prompt_to_return() -> io::Result<()> {
    print!("\nPress Enter to return to the select menu...");
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(())
}
