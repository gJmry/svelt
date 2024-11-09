mod project_creation;
mod help;

pub fn run(args: Vec<String>) {
    if !args.is_empty() {
        match args[1].as_str() {
            "init" => project_creation::main(args),
            "create" => project_creation::main(args),
            _ => {help::main().expect("Error while fetching help");}
        }
    }
}