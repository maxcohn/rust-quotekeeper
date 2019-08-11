#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate rusqlite;

use rocket::data::{Data, DataStream};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

use rusqlite::types::ToSql;
use rusqlite::{Connection, Result, NO_PARAMS};

use std::io::{Read};


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

fn create_connection(db_name: &str) -> Connection {
    Connection::open(db_name).expect("Failed to open database")
}

/// Query database for all quotes
/// 
/// # Panics
/// Panics on failure to open or read database
fn get_quotes() -> Vec<Quote>{
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

#[get("/")]
fn index() -> Template {
    // create a vector of all the quotes
    let quotes = get_quotes();

    // add them to a hashmap so we can serialize it as json with a key of "quotes"
    // and value of our list of quote objects

    // a quote serializes to {"text":"text", "author":"author"}
    // by adding it to a hashmap, we get {"quotes": [{"text":"text", "author":"author"}]}
    // which is what we need to use for the templating engine
    let mut context = HashMap::new();
    context.insert("quotes", quotes);
    
    // render the template with the given context
    return Template::render("index", &context);
}

/// Route for handling submission of new quotes
/// 
/// This route accepts post requests that contain a body in the following format:
/// {
///     text: "text",
///     author: "author"
/// }
/// 
/// The JSON is processed into a Quote struct and then added to the database
#[post("/newquote", data="<json>")]
fn new_quote(json: Data) {
    
    // read the body of the request into a string
    let mut json_str = String::new();
    match json.open().read_to_string(&mut json_str) {
        Err(_) => {
            println!("Failed to read body of POST request.");
            return
        },
        Ok(_) => println!("New quote via POST: {}", json_str),
    };

    // deserialize into Quote struct
    let q: Quote = serde_json::from_str(&json_str).expect("Unable to parse body of request");

    // insert into database
    let conn = create_connection("test.db");
    let mut stmt = conn.prepare("INSERT INTO quotes (quote, author) VALUES (?1, ?2)").unwrap();
    stmt.execute(params![&q.text, &q.author]).expect("Failed to insert quote into database");
}

#[get("/submitquote")]
fn submit_quote() -> Template {
    let context: HashMap<i32,i32> = HashMap::new();

    return Template::render("submitquote", &context);
}

#[get("/filter/name/<name>")]
fn filter_name(name: String) -> Template {
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
    let quotes: Vec<Quote> = quote_iter.map(|r| r.unwrap()).collect();

    let mut context = HashMap::new();
    context.insert("quotes", quotes);

    
    Template::render("index", &context)
}

fn main() {

    let _a = get_quotes();
    
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, submit_quote, new_quote, filter_name]) // standard routes 
        .mount("/static", StaticFiles::from("static")) // route for static content
        .launch();
}
