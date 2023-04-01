mod business;
mod config;
mod database;
mod development;
mod routes;

use actix_files as fs;
use actix_web::{web, App, HttpServer};

use mysql::{Pool, PooledConn, params, Opts, OptsBuilder};
use r2d2::Pool as R2D2Pool;
use r2d2_mysql::MySqlConnectionManager;

extern crate dotenv;

use dotenv::dotenv;
use std::env;

#[macro_use]
extern crate dotenv_codegen;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let server = config::file::get_server_config();
    let application = config::file::get_application_config();

    println!("Application running in {} mode", application.mode);
    development::machine::get_server_url_qr();

    //business::drops::scrape_drops_language("german").await;

    let url: &str = dotenv!("new_url");

    let opts  = Opts::from_url(url).unwrap();
    
    let mut builder = OptsBuilder::from_opts(opts).ssl_opts(mysql::SslOpts::default());

    let manager = MySqlConnectionManager::new(builder);
    let pool = R2D2Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(fs::Files::new("/static", "./templates/static").show_files_listing())
            .service(routes::index)
            .service(routes::setup::setup)
            .service(routes::setup::database_setup)
            .service(routes::setup::post_setup)
            .service(routes::api::json::api)
            .service(routes::api::json::get_groups)
            .service(routes::api::templates::card)
    })
    .bind((server.ip, server.port))?
    .run()
    .await
}
