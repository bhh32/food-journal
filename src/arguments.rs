use clap::{Args, arg};

/// Arguments for the add subcommand
#[derive(Debug, Clone, Args)]
pub struct AddArgs {
    #[arg(short, long, value_name = "FOOD", required = true, help = "The food that needs to be logged")]
    food: String,
    #[arg(short, long, value_name = "TIME", required = false, help = "The time the food was eaten.")]
    time: Option<String>,
    #[arg(short, long, value_name = "DATE", required = true, help = "The date the food was eaten.")]
    date: String,
    #[arg(short, long, value_name = "MEAL", required = true, help = "The meal the food was eaten during.")]
    meal: String,
}

/// Arguments for the edit subcommand
#[derive(Debug, Clone, Args)]
pub struct EditArgs {
    #[arg(short, long, value_name = "ENTRY_ID", required = true, help = "The ID of the journal entry")]
    id: String,
}

/// Arguments for the list command
#[derive(Debug, Clone, Args)]
pub struct ListArgs {
    #[arg(short, long, help = "List all journal entries.")]
    all: bool,
    #[arg(short, long, value_name = "START_DATE", requires = "end_date", help = "Start date for listed journal entries.")]
    start_date: Option<String>,
    #[arg(short, long, value_name = "END_DATE", requires = "start_date", help = "End date for listed journal entries.")]
    end_date: Option<String>,
}