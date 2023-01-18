mod development;
mod routes;
mod config;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let server = config::file::get_server_config();
    let application = config::file::get_application_config();

    println!("Application running in {} mode", application.mode);
    development::machine::get_server_url_qr();

    println!("Running server...");

    HttpServer::new(|| App::new().service(routes::index).service(routes::hello))
        .bind((server.ip, server.port))?
        .run()
        .await
}