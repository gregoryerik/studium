use actix_web::{get, Responder};
mod auth;


#[get("/")]
async fn index() -> impl Responder {
    "Hello, World!"
}

#[get("/login")]
async fn hello() -> impl Responder {
    format!("Login Route!")
}