

use crate::quote::Quote;

use rusqlite::{Connection, NO_PARAMS};

/// Name of the database file used for storing the quotes
const DB_NAME: &str = "quotes.db";

/// Query database for all quotes
/// 
/// # Errors
/// Panics on failure to open or read database
pub fn get_quotes() -> Vec<Quote>{
    let conn = create_connection(DB_NAME);
    
    // query all quotes. We assume that the quote's table exists
    let mut stmt = conn.prepare("SELECT * FROM quotes")
        .expect("Error reading from quotes. Check if the database exists.");

    // get an iterator over Result<Quote> (required by rusqlite)
    let quote_iter = stmt.query_map(NO_PARAMS, |row|
        Ok(Quote {
            text: row.get(0).unwrap(),
            author: row.get(1).unwrap()
        })
    ).expect("Error reading from quotes. Check if the database exists.");

    // convert iterator into vector
    quote_iter.map(|r| r.unwrap()).collect()
}

/// Query database for all quotes that contain the given text
pub fn filter_text(text: String) -> Vec<Quote>{
    let conn = create_connection(DB_NAME);

    // create the pattern matching string for the LIKE
    let text = format!("%{}%", text);

    //println!("{}", name);

    let mut stmt = conn.prepare("SELECT * FROM quotes WHERE quote LIKE ?")
        .expect("Error reading from quotes. Check if the database exists.");
    
    // get an iterator over Result<Quote> (required by rusqlite)
    let quote_iter = stmt.query_map(params![&text], |row|
        Ok(Quote {
            text: row.get(0).unwrap(),
            author: row.get(1).unwrap()
        })
    ).expect("Error reading from quotes. Check if the database exists.");

    // convert iterator into vector
    quote_iter.map(|r| r.unwrap()).collect()
}

/// Query the database for all quotes that contain the given text
pub fn filter_name(name: String) -> Vec<Quote> {
    let conn = create_connection(DB_NAME);

    // create the pattern matching string for the LIKE
    let name = format!("%{}%", name);

    //println!("{}", name);

    let mut stmt = conn.prepare("SELECT * FROM quotes WHERE author LIKE ?")
        .expect("Error reading from quotes. Check if the database exists.");
    
    // get an iterator over Result<Quote> (required by rusqlite)
    let quote_iter = stmt.query_map(params![&name], |row|
        Ok(Quote {
            text: row.get(0).unwrap(),
            author: row.get(1).unwrap()
        })
    ).expect("Error reading from quotes. Check if the database exists.");

    // convert iterator into vector
    quote_iter.map(|r| r.unwrap()).collect()
}

/// Insert the given Quote struct into the database
pub fn insert_quote(q: Quote) {
    let conn = create_connection(DB_NAME);
    let mut stmt = conn.prepare("INSERT INTO quotes (quote, author) VALUES (?1, ?2)").unwrap();
    stmt.execute(params![&q.text, &q.author]).expect("Failed to insert quote into database");
}


/// Create and return a connection to the given database file
fn create_connection(db_name: &str) -> Connection {
    Connection::open(db_name).expect("Failed to open database")
}

/// Check if the 'quotes' table exists in the database. If it doesn't, create it.
pub fn check_for_table() {
    let conn = create_connection(DB_NAME);

    // query the database to check if the 'quotes' table exists
    let mut stmt = conn.prepare("SELECT count(*) FROM sqlite_master WHERE type='table' AND name='quotes'")
        .expect("Error while reading all database tables");
    
    let count: i32 = stmt.query_row(NO_PARAMS, |row| row.get(0)).unwrap();

    // if the 'quotes' table doens't exist, create it
    if count == 0 {
        conn.execute("CREATE TABLE quotes (quote TEXT, author TEXT)", NO_PARAMS)
            .expect("Failed to createa 'quotes' table.");
    }

}
