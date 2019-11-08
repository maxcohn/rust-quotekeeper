
#[macro_use] extern crate rusqlite;
#[macro_use] extern crate tera;

mod query;
mod quote;
use quote::Quote;

use actix_web::{
    web, App, HttpServer, Error, HttpResponse,
};
use actix_files;

use actix_web::http::StatusCode;


/// Load and output all quotes
fn index(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    // a quote serializes to {"text":"text", "author":"author"}
    // by adding it to a context, we get {"quotes": [{"text":"text", "author":"author"}]}
    // which is what we need to use for the templating engine
    let mut ctx = tera::Context::new();
    ctx.insert("quotes", &query::get_quotes());

    // render the template with the current context
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
fn new_quote(json: web::Json<Quote>) -> HttpResponse {
    query::insert_quote(json.into_inner());

    HttpResponse::new(StatusCode::from_u16(200).unwrap())
}

/// Filters output page based on quote text
fn filter_text(tmpl: web::Data<tera::Tera>, path: web::Path<(String,)>) -> HttpResponse {
    let text = path.0.clone();

    let mut ctx = tera::Context::new();
    ctx.insert("quotes", &query::filter_text(text));

    let page = tmpl.render("index.html.tera", &ctx).unwrap();

    HttpResponse::Ok().content_type("text/html").body(page)
}

/// Filters output page based on quote author's name
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
            .service(actix_files::Files::new("/static", "./static"))
            .route("/", web::get().to(index))
            .route("/filter/name/{name}",web::get().to(filter_name))
            .route("/filter/text/{text}", web::get().to(filter_text))
            .route("/newquote", web::post().to(new_quote))
            .route("/submitquote", web::get().to(|| actix_files::NamedFile::open("templates/submitquote.html")))

    }).bind("127.0.0.1:8003").expect("Failed to bind socket")
        .run().expect("Failed to start server");

}
