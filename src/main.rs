mod controller;
mod models;

use controller::cli;

#[cfg(windows)]
pub const NPM: &'static str = "npm.cmd";
#[cfg(windows)]
pub const NPX: &'static str = "npx.cmd";

#[cfg(not(windows))]
pub const NPM: &'static str = "npm";
#[cfg(not(windows))]
pub const NPX: &'static str = "npx";

fn main() {
    cli::run();
}
