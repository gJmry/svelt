use std::process::Command;
use crate::NPM;

pub fn main(){
    Command::new(NPM)
        .arg("start")
        .status()
        .expect("Failed to run start command");
}