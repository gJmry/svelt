
mod cli;
mod utils;
mod commands;

use std::env;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";
#[cfg(windows)]
pub const NPX: &'static str = "npx.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";
#[cfg(not(windows))]
pub const NPX: &'static str = "npx";

fn main() {
    let args: Vec<String> = env::args().collect();
    cli::run(args);
}

