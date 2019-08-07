#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rusqlite;

use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};

#[derive(Serialize, Deserialize, Debug)]
struct Quote {
    text: String,
    author: String
}

impl Quote {
    fn new(text: &str, author: &str) -> Quote {
        Quote {
            text: text.to_string(),
            author: author.to_string()
        }
    }
}

/// Query database for all quotes
fn get_quotes() -> Vec<Quote>{
    let conn = Connection::open("test.db").expect("Failed to open database");

    // create 'quotes' table if it doesn't exist.
    if conn.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='quotes'", params![]).is_err() {
        conn.execute("CREATE TABLE quotes (quote TEXT NOT NULL, author TEXT NOT NULL);", params![])
            .expect("Failed to create table 'quotes'");
    }
    
    
    // insert quotes
    conn.execute("INSERT INTO quotes (quote, author) VALUES (?1,?2)", params!["asd","dsa"]).unwrap();


    Vec::new()
}

#[get("/")]
fn index() -> Template {
    // create a vector of all the quotes
    let quotes = vec![Quote::new("t_asd", "auth")];
    // add them to a hashmap so we can serialize it as json with a key of "quotes"
    // and value of our list of quote objects
    let mut hm = HashMap::new();
    hm.insert("quotes", quotes);
    
    // render the template with the given context
    return Template::render("index", &hm);
}

#[post("/newquote")]
fn new_quote() {
    //TODO get body of the POST request

    // insert into database
}

fn main() {

    let _a = get_quotes();
    
    rocket::ignite().attach(Template::fairing())
        .mount("/", routes![index]) // standard routes 
        .mount("/static", StaticFiles::from("static")) // route for static content
        .launch();
}
