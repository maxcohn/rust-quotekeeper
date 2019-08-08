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
/// 
/// # Panics
/// Panics on failure to open or read database
fn get_quotes() -> Vec<Quote>{
    let conn = Connection::open("test.db").expect("Failed to open database");
    
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

#[get("/")]
fn index() -> Template {
    // create a vector of all the quotes
    let quotes = get_quotes();

    // add them to a hashmap so we can serialize it as json with a key of "quotes"
    // and value of our list of quote objects

    // a quote serializes to {"text":"text", "author":"author"}
    // by adding it to a hashmap, we get {"quotes": [{"text":"text", "author":"author"}]}
    // which is what we need to use for the templating engine
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
