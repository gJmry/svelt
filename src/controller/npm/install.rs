use std::process::Command;
use std::process::exit;
use crate::NPM;

pub fn main() {
    if !Command::new(NPM)
        .arg("install")
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!("Error running npm install");
        exit(1);
    }
}
