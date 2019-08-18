#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rusqlite;


mod query;
mod quote;
use quote::Quote;

use rocket::data::{Data};
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;

use std::collections::HashMap;
use std::io::{Read};





#[get("/")]
fn index() -> Template {
    // create a vector of all the quotes
    let quotes = query::get_quotes();

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
    query::insert_quote(q);
}

#[get("/submitquote")]
fn submit_quote() -> Template {
    let context: HashMap<i32,i32> = HashMap::new();

    return Template::render("submitquote", &context);
}

#[get("/filter/name/<name>")]
fn filter_name(name: String) -> Template {
    let quotes = query::filter_name(name);

    let mut context = HashMap::new();
    context.insert("quotes", quotes);

    
    Template::render("index", &context)
}

#[get("/filter/text/<text>")]
fn filter_text(text: String) -> Template {
    
    let quotes = query::filter_text(text);
    let mut context = HashMap::new();
    context.insert("quotes", quotes);

    
    Template::render("index", &context)
}

fn main() {
    query::check_for_table("test.db");
    rocket::ignite()
        .attach(Template::fairing())
        .mount("/", routes![index, submit_quote, new_quote, filter_name, filter_text]) // standard routes 
        .mount("/static", StaticFiles::from("static")) // route for static content
        .launch();
}
