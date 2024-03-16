use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Clone, Deserialize)]
pub enum Meal {
    BREAKFAST,
    BRUNCH,
    LUNCH,
    LINNER,
    DINNER
}

#[derive(Debug, Deserialize, Clone)]
pub struct Entry {
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "Meal")]
    meal: Meal,
    #[serde(rename = "Food")]
    food: String,
    #[serde(rename = "Time")]
    time: Option<String>,
    #[serde(rename = "Date")]
    date: String,
}

impl Entry {
    pub fn add(meal: Meal, food: String, time: Option<String>, date: String) -> Result<(), Box<dyn Error>>{
        let mut record_container = Vec::<Entry>::new();

        let journal = std::fs::File::open("food_journal.csv")?;
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_reader(journal);

        for result in rdr.deserialize() {
            let record: Entry = result?;
            record_container.push(record);
        }

        let last_id = record_container[record_container.len() - 1].id;

        let new_entry: Entry = Entry {
            id: last_id + 1,
            meal,
            food,
            time,
            date
        };

        record_container.push(new_entry);

        Ok(())
    }

    pub fn edit(id: u64) {
        todo!();
    }
}

pub fn list_all() {
    todo!();
}

pub fn list_range(start: String, end: String) {
    todo!();
}

pub fn list_single(id: u64) {

}