#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
//use rocket_contrib::templates::Context;
use serde::{Serialize, Deserialize};





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


#[get("/")]
fn index() -> Template {
    // create a vector of all the quotes
    let quotes = vec![Quote::new("t_asd", "auth")];
    // add them to a hashmap so we can serialize it as json with a key of "quotes"
    // and value of our list of quote objects
    let mut hm = std::collections::HashMap::new();
    hm.insert("quotes", quotes);
    
    // render the template with the given context
    Template::render("index", &hm)
}

fn main() {
    let quotes = vec![Quote::new("t_asd", "auth")];
    let mut hm = std::collections::HashMap::new();
    hm.insert("quotes", quotes);
    let serial = serde_json::to_string(&hm).unwrap();
    println!("{}", serial);
    
    
    
    rocket::ignite().attach(Template::fairing())
        .mount("/", routes![index])
        .mount("/static", StaticFiles::from("static")).launch();
}
