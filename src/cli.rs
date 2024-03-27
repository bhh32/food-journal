use clap::{Parser, Subcommand};
use crate::arguments::{AddArgs, EditArgs, ListArgs};

#[derive(Parser)]
#[command(version, about, long_about = None, author = "Bryan Hyland <bryan.hyland32@gmail.com>")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

/// Commands
#[derive(Debug, Subcommand)]
pub enum Commands {
    #[command(about = "Add items to the journal.")]
    Add(AddArgs),
    #[command(about = "Edit items in the journal.")]
    Edit(EditArgs),
    #[command(about = "List items in the journal.")]
    List(ListArgs),
}