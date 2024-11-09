use std::process::{exit, Command};
use cliclack::*;
use crate::NPX;

pub fn main(args: Vec<String>) {
    let project_name = if args.len() > 2 {
        args[2].clone()
    } else {
        let name: String = input("What is your project name?")
            .placeholder("./sparkling-solid")
            .validate(|input: &String| {
                if input.is_empty() {
                    Err("Please enter a name.")
                } else {
                    Ok(())
                }
            })
            .interact()
            .unwrap();

        name
    };

    make_svelte_project(project_name);
}

fn make_svelte_project(project_name: String) {
    let status = Command::new(NPX)
        .arg("sv")
        .arg("create")
        .arg(project_name)
        .status()
        .expect("Failed to execute command");

    if !status.success() {
        eprintln!("Error trying to use NPX command");
        exit(1);
    }
}
