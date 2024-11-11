use std::process::{exit, Command};
use crate::NPM;

pub fn main(env: Option<String>){
    let mut npm_command = Command::new(NPM);
    npm_command.arg("run");

    if let Some(env) = env{
        npm_command.arg(env);
    }

    if !npm_command
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        eprintln!("Error running npm run");
        exit(1);
    }

}