use crate::utils;

mod project_creation;
mod help;
mod version_check;

pub fn run(args: Vec<String>) {
    if !args.is_empty() {
        if args.len() == 1 {
            help::main().expect("Error while fetching help");
        } else {
            match args[1].as_str() {
                "init" => project_creation::main(args),
                "create" => project_creation::main(args),
                "version" => version_check::main(),
                _ => { help::main().expect("Error while fetching help"); }
            }
        }
    }
}