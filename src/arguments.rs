use clap::{Args, arg};

#[derive(Debug, Clone, Args)]
pub struct AddArgs {
    #[arg(short, long, value_name = "FOOD", required = true, help = "The food that needs to be logged")]
    food: String,
}