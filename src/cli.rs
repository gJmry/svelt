mod project_creation;

pub fn run(args: Vec<String>) {
    if !args.is_empty() {
        match args[1].as_str() {
            "init" => project_creation::main(args),
            "create" => project_creation::main(args),
            ""
            _ => {}
        }
    }
}