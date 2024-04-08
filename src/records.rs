use rusqlite::{params, Connection, Error};
use chrono::{Datelike, Days, NaiveDate};

/// Defines the type of meal that can be entered into the jounal.
#[derive(Debug, Clone)]
pub enum Meal {
    BREAKFAST,
    BRUNCH,
    LUNCH,
    LINNER,
    DINNER,
    SNACK,
}

/// Defines the data of a jounal entry.
#[derive(Debug, Clone)]
pub struct Entry {
    id: i32,
    meal: String,
    food: String,
    time: String,
    date: String,
}

// The jounal database SQLite file path/name.
const DB_PATH: &str = "/home/bryan/Documents/food_journal/journal.db";

pub fn edit(_id: u64) {
    todo!();
}

/// Adds an entry to the jounal database.
pub fn add(meal: Meal, food: String, time: String, date: String)  -> Result<(), Error> {
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

    // Get the last jounal entry in the jounal database.
    let last_entry = db.query_row("SELECT * FROM journal ORDER BY id DESC LIMIT 1", [], |row| {
        Ok(Entry {
            id: row.get(0)?,
            meal: row.get(1)?,
            food: row.get(2)?,
            time: row.get(3)?,
            date: row.get(4)?,
        })
       // If the database is empty, create a "default" Entry that has the id of 0.
    }).unwrap_or(Entry {
        id: 0,
        meal: String::new(),
        food: String::new(),
        time: String::new(),
        date: String::new(),
    });

    // Create a new id from the id of last_entry.
    let new_id = last_entry.id + 1;
    
    // Create the new Entry from the entered CLI information
    let new_entry = Entry {
        id: new_id,
        meal: match meal {
            Meal::BREAKFAST => String::from("BREAKFAST"),
            Meal::BRUNCH => String::from("BRUNCH"),
            Meal::LUNCH => String::from("LUNCH"),
            Meal::LINNER => String::from("LINNER"),
            Meal::DINNER => String::from("DINNER"),
            Meal::SNACK => String::from("SNACK"),
        },
        food,
        time,
        date,
    };

    // Insert the Entry into the journal database.
    db.execute(
        "INSERT INTO journal (id, meal, food, time, date) VALUES (?1, ?2, ?3, ?4, ?5)",
        params!(&new_entry.id, &new_entry.meal, &new_entry.food, &new_entry.time, &new_entry.date),
    )?;

    // Return an Ok(()) Result
    Ok(())
}

/// List all jounal entries.
pub fn list_all() -> Result<(), Error> {
    // Open the database file
    let db = Connection::open(DB_PATH)?;

    // Prepare the SQL query.
    let mut req = db.prepare("SELECT * FROM journal")?;

    // Loop through each jounal database row and put them in a MappedRow of Entry data
    let req_iter = req.query_map([], |row| {
        Ok(Entry {
            id: row.get(0)?,
            meal: row.get(1)?,
            food: row.get(2)?,
            time: row.get(3)?,
            date: row.get(4)?,
        })
    }).expect("Could not print out the database!");

    // Print the current jounal entry using the database row data.
    for entry in req_iter {
        match entry {
            Ok(ent) => {
                println!("{} {} {} {} {}", ent.id, ent.date, ent.time, ent.meal, ent.food);
            },
            Err(e) => eprint!("Bad Data!!\n{e}"),
        }
    }

    // Return an Ok(()) Result
    Ok(())
}

pub fn list_range(start: String, end: String) -> Result<(), Error> {
    // Open the database file
    let db = Connection::open(DB_PATH)?;
    
    match (NaiveDate::parse_from_str(&start, "%d%b%Y"), NaiveDate::parse_from_str(&end, "%d%b%Y")) {
        (Ok(start_date), Ok(end_date)) => {            
            // Subtract end from start
            let diff = end_date.signed_duration_since(start_date);

            println!("diff: {}", diff.num_days());

            for day in 0..diff.num_days() {
                let query_date = match start_date.checked_add_days(Days::new(5)) {
                    // Return the date if it's valid
                    Some(date) => {
                        let month = match date.month0() {
                            1 => "Jan",
                            2 => "Feb",
                            3 => "Mar",
                            4 => "Apr",
                            5 => "May",
                            6 => "Jun",
                            7 => "Jul",
                            8 => "Aug",
                            9 => "Sep",
                            10 => "Oct",
                            11 => "Nov",
                            12 => "Dec",
                            _ => ""
                        };

                        let (_, yr) = date.year_ce();

                        let searchable_date = format!("{}{}{}", date.day(), month, yr);
                        println!("searchable_date: {searchable_date}");

                        searchable_date
                    },
                    // Continue the loop if it's not a valid date.
                    None => {
                        println!("Debug: continuing...");
                        continue;
                    }
                };

                let mut query = db.prepare("SELECT * FROM journal WHERE date = (date) VALUES (?1)")?;
                let mut rows = query.query([query_date.to_string()])?;

                while let Some(row) = rows.next()? {
                    let id: u32 = row.get(0)?;
                    let meal: String = row.get(1)?;
                    let food: String = row.get(2)?;
                    let time: String = row.get(3)?;
                    let date: String = row.get(4)?;
            
                    println!("{id} {date} {time} {meal} {food}");
                }
            }
        },
        (Ok(_), Err(e)) => {
            eprintln!("End date incorrect!\n{e}");
            return Err(Error::QueryReturnedNoRows);
        },
        (Err(e), Ok(_)) => {
            eprintln!("Start date incorrect!\n{e}");
            return Err(Error::QueryReturnedNoRows);
        }
        (Err(e1), Err(e2)) => {
            eprintln!("{e1}\n{e2}");
            return Err(Error::QueryReturnedNoRows);
        }
    }

    Ok(())
}

pub fn list_single(id: i32) -> Result<(), Error> {
    // Open the database file
    let db = Connection::open(DB_PATH)?;

    // Prepare the SQL query.
    let mut req = db.prepare("SELECT * FROM journal WHERE id = (id) VALUES (?1)")?;
    let mut rows = req.query([id])?;

    while let Some(row) = rows.next()? {
        // Error checking not working, fix...
        if id != row.get(0)? {
            eprintln!("Entry for that ID not found!");
            break;
        }

        let id: u32 = row.get(0)?;
        let meal: String = row.get(1)?;
        let food: String = row.get(2)?;
        let time: String = row.get(3)?;
        let date: String = row.get(4)?;

        println!("{id} {date} {time} {meal} {food}");
    };

    Ok(())
}