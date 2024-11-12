use crate::models::commands::Commands;
use clap::Parser;

#[derive(Parser)]
#[command(author = "Jérémy Girard", version = "0.1", about = "A CLI made in Rust for Svelte", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}
