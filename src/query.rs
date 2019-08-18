

use crate::quote::Quote;

use rusqlite::{Connection, NO_PARAMS};



/// Query database for all quotes
/// 
/// # Panics
/// Panics on failure to open or read database
pub fn get_quotes() -> Vec<Quote>{
    let conn = create_connection("test.db");
    
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

pub fn filter_text(text: String) -> Vec<Quote>{
    let conn = create_connection("test.db");

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

pub fn filter_name(name: String) -> Vec<Quote> {
    let conn = create_connection("test.db");

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

pub fn insert_quote(q: Quote) {
    let conn = create_connection("test.db");
    let mut stmt = conn.prepare("INSERT INTO quotes (quote, author) VALUES (?1, ?2)").unwrap();
    stmt.execute(params![&q.text, &q.author]).expect("Failed to insert quote into database");
}

fn create_connection(db_name: &str) -> Connection {
    Connection::open(db_name).expect("Failed to open database")
}

//TODO check for table existence, if it does, do nothing, otherwise, create a new one
pub fn check_for_table(db_name: &str) {
    let conn = create_connection(db_name);

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
/* 
# check if the 'quotes' table exists
    with sqlite3.connect(FILE_PATH) as conn:        
        # create database connection
        cursor = conn.cursor()
        cursor.execute("select count(*) from sqlite_master where type='table' and name='quotes'")
        if cursor.fetchone()[0] == 0:
            # table doesn't exist, so create one
            cursor.execute("create table quotes (quote text, author text)")
            cursor.commit() */