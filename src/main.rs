mod cli;
pub mod arguments;
pub mod records;

use cli::{Cli, Commands};
use records::*;
use clap::{Parser};

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
        Commands::List(args) => {
            if args.all {
                let _ = list_all();
                return;
            }
            let id = match args.id {
                Some(found_id) => {
                    println!("Debug found_id: {found_id}");

                    if found_id > 0 {
                        let _ = list_single(found_id);
                        return;
                    }
                },
                None => {
                    
                }
            };
            let (start_date, end_date) = (args.start_date, args.end_date);
            
             let _start_date = match start_date {
                Some(s_date) => s_date,
                None => String::new(),
             };

             let _end_date = match end_date {
                Some(e_date) => e_date,
                None => String::new()
             };
        }
    }
}