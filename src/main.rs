mod development;
mod routes;
mod config;
mod database;

use actix_web::{App, HttpServer};
use actix_files as fs;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let server = config::file::get_server_config();
    let application = config::file::get_application_config();

    println!("Application running in {} mode", application.mode);
    development::machine::get_server_url_qr();

    println!("Running server...");

    HttpServer::new(|| App::new()
        .service(fs::Files::new("/static", "./templates/static").show_files_listing())
        .service(routes::index)
        .service(routes::setup::setup)
        .service(routes::setup::database_setup)
        .service(routes::api::json::api)
    )
        .bind((server.ip, server.port))?
        .run()
        .await
}