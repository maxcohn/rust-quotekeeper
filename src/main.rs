
#[macro_use] extern crate rusqlite;
#[macro_use] extern crate tera;

mod query;
mod quote;
use quote::Quote;

use actix_web::{web, App, HttpServer, Responder, Error, HttpResponse, HttpRequest};
//use tera;
use std::collections::HashMap;
use std::io::{Read};
use actix_web::http::StatusCode;


//#[get("/")]
fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    /*
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
    */
    let mut ctx = tera::Context::new();
    ctx.insert("quotes", &query::get_quotes());

    let page = tmpl.render("index.html.tera", &ctx).unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(page))
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
//#[post("/newquote", data="<json>")]
/*
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

//#[get("/submitquote")]
fn submit_quote() -> Template {
    let context: HashMap<i32,i32> = HashMap::new();

    return Template::render("submitquote", &context);
}


//#[get("/filter/text/<text>")]
fn filter_text(text: String) -> Template {

    let quotes = query::filter_text(text);
    let mut context = HashMap::new();
    context.insert("quotes", quotes);


    Template::render("index", &context)
}
*/

fn filter_name(tmpl: web::Data<tera::Tera>, path: web::Path<(String,)>) -> HttpResponse {
    let name = path.0.clone();

    let mut ctx = tera::Context::new();
    ctx.insert("quotes", &query::filter_name(name));

    let page = tmpl.render("index.html.tera", &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(page)
}


fn main() {
    // check to make sure the given table exists
    query::check_for_table();

    HttpServer::new(|| {
        let tera = compile_templates!(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*"));

        App::new()
            .data(tera)
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/filter/name/{name}").route(web::get().to(filter_name)))

    }).bind("127.0.0.1:8003").unwrap().run();;

}
