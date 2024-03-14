mod cli;
pub mod arguments;

use cli::Cli;
use clap::Parser;

fn main() {
    let _cli = Cli::parse();
}