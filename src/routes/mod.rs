use actix_web::{get, Responder, HttpResponse};
use askama::Template;

//Modules
mod auth;
mod templates;
pub mod setup;


#[get("/")]
async fn index() -> actix_web::Result<HttpResponse> {
    let index_template = templates::IndexTemplate {
        name: "Greg" // Currently used in place of auth/account system 
    }.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(index_template))
}

#[get("/login")]
async fn hello() -> impl Responder {
    format!("Login Route!")
}