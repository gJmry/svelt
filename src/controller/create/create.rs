use cliclack::{input, select, Validate};
use crate::controller::create::match_schematic::match_schematic;
use crate::controller::utils::is_svelte_project;
use crate::models::schematic::Schematic;

pub fn main(schematic: Option<String>, name: Option<String>) {
    if (!is_svelte_project::main()) {
        println!("Please use this command when you are in a Svelte Project");
    } else {
        let schematic = schematic.unwrap_or_else(choose_schematic);
        let name = name.unwrap_or_else(|| choose_name(schematic.clone()));

        match_schematic(schematic, name);
    }
}

fn choose_schematic() -> String {
    let schematics = Schematic::all();

    let schematic = select("Pick a schematic".to_string())
        .initial_value("component")
        .items(
            &schematics
                .iter()
                .map(|schematic| {
                    (
                        schematic.command(),
                        schematic.label(),
                        schematic.hint(),
                    )
                })
                .collect::<Vec<(&str, &str, &str)>>()
                .as_slice()
        )
        .interact()
        .unwrap_or_else(|_| {
            panic!("Failed to choose a schematic");
        });

    schematic.to_string()
}

fn choose_name(schematic: String) -> String {
    input("What is your schematic name?")
        .placeholder(&format!("my{}", make_first_letter_uppercase(&schematic)))
        .validate(|input: &String| {
            if input.is_empty() {
                Err("Please enter a name.")
            } else {
                Ok(())
            }
        })
        .interact()
        .unwrap_or_else(|_| {
            panic!("Failed to get user input for schematic name");
        })
}

fn make_first_letter_uppercase(s: &str) -> String {
    if let Some(first_char) = s.chars().next() {
        first_char.to_uppercase().collect::<String>() + &s[1..]
    } else {
        String::new()
    }
}

