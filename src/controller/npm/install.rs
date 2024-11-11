use std::process::Command;
use std::process::exit;
use crate::NPM;

pub fn main(package: Option<String>, flag: Option<String>) {
    let mut npm_command = Command::new(NPM);
    npm_command.arg("install");

    if let Some(package) = package {
        npm_command.arg(package);
    }

    if let Some(flag) = flag {
        npm_command.arg(flag);
    }

    if !npm_command
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!("Error running npm install");
        exit(1);
    }
}
