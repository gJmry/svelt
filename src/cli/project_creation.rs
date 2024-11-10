mod add_ui_toolkit;

use std::process::{exit, Command};
use cliclack::*;
use crate::{NPM, NPX};

pub fn main(project_name: Option<String>, ui_toolkit_name: Option<String>) {
    let project_name = project_name.unwrap_or_else(get_project_name);
    let ui_toolkit_name = ui_toolkit_name.unwrap_or_else(get_ui_toolkit_name);

    make_svelte_project(project_name);
    add_ui_toolkit(ui_toolkit_name);
}

fn get_project_name() -> String {
    intro("Project Creation").expect("Error while prompting intro");
    input("What is your project name?")
        .placeholder("./sparkling-solid")
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter a name.")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap()
}


fn get_ui_toolkit_name() -> String {
    let ui_toolkit_name: &str = select("Pick a UI toolkit".to_string())
        .initial_value("None")
        .item("None", "None", "You don't want an UI Toolkit because you are better")
        .item("Tailwind", "Tailwind", "A utility-first CSS framework for rapid UI development.")
        .item("Bootstrap", "Bootstrap", "A popular CSS framework with a lot of pre-built components.")
        .item("Skeleton", "Skeleton", "A lightweight CSS framework for minimalistic designs.")
        .item("Flowbite", "Flowbite", "A UI kit based on Tailwind CSS with ready-to-use components.")
        .interact()
        .unwrap();

    ui_toolkit_name.parse().unwrap()
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

fn add_ui_toolkit(toolkit_name: String) {
    match toolkit_name.as_str() {
        "skeleton" => add_ui_toolkit::skeleton(),
        "flowbite" => add_ui_toolkit::flowbite(),
        "tailwind" => add_ui_toolkit::tailwind(),
        "bootstrap" => add_ui_toolkit::bootstrap(),
        _ => {}
    }
}
