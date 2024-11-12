use crate::NPM;
use std::process::Command;

pub fn main() {
    Command::new(NPM)
        .arg("start")
        .status()
        .expect("Failed to run start command");
}
