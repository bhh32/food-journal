use clap::{Parser, Subcommand};
use crate::arguments::AddArgs;

#[derive(Parser)]
#[command(version, about, long_about = None, author = "Bryan Hyland <bryan.hyland32@gmail.com>")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    #[command(about = "Add items to the journal.")]
    Add(AddArgs),
}