pub enum Meal {
    BREAKFAST,
    BRUNCH,
    LUNCH,
    LINNER,
    DINNER
}

pub struct Entry {
    id: u64,
    meal: Meal,
    food: String,
    time: Option<String>,
    date: String,
}

impl Entry {
    pub fn add(meal: Meal, food: String, time: Option<String>, date: String) {
        todo!();
    }

    pub fn edit(&mut self) {
        todo!();
    }
}

pub fn list_all() {
    todo!();
}

pub fn list_range(start: String, end: String) {
    todo!();
}