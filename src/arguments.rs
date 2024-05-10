use clap::{Args, arg};

/// Arguments for the add subcommand
#[derive(Debug, Clone, Args)]
pub struct AddArgs {
    #[arg(short, long, value_name = "FOOD", required = true, help = "The food that needs to be logged")]
    pub food: String,
    #[arg(short, long, value_name = "24HR (2400)", required = false, help = "The time the food was eaten.")]
    pub time: Option<String>,
        #[arg(short, long, value_name = "mm/dd/yyyy", required = true, help = "The date the food was eaten.")]
    pub date: String,
    #[arg(short, long, value_name = "MEAL", required = true, value_parser = ["breakfast", "brunch", "lunch", "linner", "dinner", "snack"], help = "The meal the food was eaten during.")]
    pub meal: String,
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
    pub all: bool,
    #[arg(short, long, required = false, help = "List a single journal entry with the listed id.")]
    pub id: Option<i32>,
    #[arg(short, long, value_name = "START_DATE", requires = "end_date", help = "Start date for listed journal entries.")]
    pub start_date: Option<String>,
    #[arg(short, long, value_name = "END_DATE", requires = "start_date", help = "End date for listed journal entries.")]
    pub end_date: Option<String>,
}
