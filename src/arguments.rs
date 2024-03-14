use clap::{Args, arg};

#[derive(Debug, Clone, Args)]
pub struct AddArgs {
    #[arg(short, long, value_name = "FOOD", required = true, help = "The food that needs to be logged")]
    food: String,
    #[arg(short, long, value_name = "TIME", required = false, help = "The time the food was eaten.")]
    time: String,
    #[arg(short, long, value_name = "DATE", required = true, help = "The date the food was eaten.")]
    date: String,
    #[arg(short, long, value_name = "MEAL", required = true, help = "The meal the food was eaten during.")]
    meal: String,
}