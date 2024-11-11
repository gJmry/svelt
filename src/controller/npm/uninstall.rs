use std::process::Command;
use std::process::exit;
use crate::NPM;

pub fn main(package: String) {
    let mut npm_command = Command::new(NPM);
    npm_command.arg("uninstall");
    npm_command.arg(package);

    if !npm_command
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!("Error running npm uninstall");
        exit(1);
    }
}
