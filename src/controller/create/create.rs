use cliclack::{input, select, Validate};
use crate::models::schematic::Schematic;

pub fn main(schematic: Option<String>, name: Option<String>) {
    let schematic = schematic.unwrap_or_else(choose_schematic);
    let name = name.unwrap_or_else(|| choose_name(schematic.clone()));

    println!("Chosen schematic: {}", schematic);
    println!("Chosen name: {}", name);
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
        .unwrap();

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
        .unwrap()
}

fn make_first_letter_uppercase(s: &str) -> String {
    if let Some(first_char) = s.chars().next() {
        first_char.to_uppercase().collect::<String>() + &s[1..]
    } else {
        String::new()
    }
}