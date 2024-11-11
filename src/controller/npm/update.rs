use std::process::{exit, Command};
use crate::NPM;

pub fn main(package: Option<String>) {
    let mut npm_command = Command::new(NPM);
    npm_command.arg("update");

    if let Some(package) = package {
        npm_command.arg(package);
    }

    if !npm_command
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!("Error running npm update");
        exit(1);
    }
}