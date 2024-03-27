use rusqlite::{params, Connection, Error};

#[derive(Debug, Clone)]
pub enum Meal {
    BREAKFAST,
    BRUNCH,
    LUNCH,
    LINNER,
    DINNER,
    SNACK,
}

#[derive(Debug, Clone)]
pub struct Entry {
    id: i64,
    meal: String,
    food: String,
    time: String,
    date: String,
}

const DB_PATH: &str = "./journal.db";

pub fn edit(_id: u64) {
    todo!();
}

pub fn add(meal: Meal, food: String, time: String, date: String)  -> Result<(), Error> {
    

    let _rng = rand::thread_rng();

    

    // Create the database if it doesn't exist
    let db = Connection::open(DB_PATH).expect("Database connected to...");
    
    // Create table if it doesn't exist.
    db.execute("CREATE TABLE IF NOT EXISTS journal (
        id INTEGER PRIMARY KEY,
        meal TEXT NOT NULL,
        food TEXT NOT NULL,
        time TEXT NULL,
        date TEXT NOT NULL
    )", [],)?;

    let new_id = db.query_row("SELECT id FROM journal ORDER BY id DESC LIMIT 1", [], |row| {
        match row.get(0)? {
            Ok(val) => val + 1,
            Err(e) => {
                eprintln!("Error getting last row id...");
                panic!();
            },
        }
    })?;
        
    let new_entry = Entry {
        id: new_id,
        meal: match meal {
            Meal::BREAKFAST => String::from("breakfast"),
            Meal::BRUNCH => String::from("brunch"),
            Meal::LUNCH => String::from("lunch"),
            Meal::LINNER => String::from("linner"),
            Meal::DINNER => String::from("dinner"),
            Meal::SNACK => String::from("snack"),
        },
        food,
        time,
        date,
    };

    db.execute(
        "INSERT INTO journal (id, meal, food, time, date) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!(&new_entry.id, &new_entry.meal, &new_entry.food, &new_entry.time, &new_entry.date),
    )?;

    println!("Debug: Data inserted!");

    Ok(())
}

pub fn list_all() -> Result<(), Error> {
    let db = Connection::open(DB_PATH)?;

    let mut req = db.prepare("SELECT * FROM journal")?;

    let req_iter = req.query_map([], |row| {
        Ok(Entry {
            id: row.get(0)?,
            meal: row.get(1)?,
            food: row.get(2)?,
            time: row.get(3)?,
            date: row.get(4)?,
        })
    }).expect("Could not print out the database!");

    for entry in req_iter {
        match entry {
            Ok(ent) => {
                println!("{} {} {} {} {}", ent.id, ent.meal, ent.food, ent.time, ent.date);
            },
            Err(e) => eprint!("Bad Data!!\n{e}"),
        }

    }

    Ok(())
}

pub fn list_range(_start: String, _end: String) {
    todo!();
}

pub fn list_single(_id: u64) {
    todo!();
}