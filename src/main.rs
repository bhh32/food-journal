mod cli;
pub mod arguments;
pub mod records;

use cli::{Cli, Commands};
use records::*;
use clap::Parser;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add(args) => {
            let meal = match args.meal.to_lowercase().as_str() {
                "breakfast" => Meal::BREAKFAST,
                "brunch" => Meal::BRUNCH,
                "lunch" => Meal::LUNCH,
                "linner" => Meal::LINNER,
                "dinner" => Meal::DINNER,
                "snack" => Meal::SNACK,
                _ => {
                    eprintln!("Incorrect meal argument entered, please try again!");
                    panic!();
                }
            };

            let _ = add(meal, args.food, String::from(""), args.date);
        },
        Commands::Edit(args) => {
            println!("{args:?}");
        },
        Commands::List(_args) => {
            let _ = list_all();
        }
    }
}