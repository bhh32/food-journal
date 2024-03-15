mod cli;
pub mod arguments;
pub mod records;

use cli::Cli;
use clap::Parser;

fn main() {
    let _cli = Cli::parse();
}