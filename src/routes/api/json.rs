use actix_web::{get, Responder, web};
use serde::Serialize;

use crate::database::local;


/// All of the api routes that could be requested. Enum through here to match to the correct function
#[derive(Serialize)]
pub enum ApiRoute{
    DatabaseExists,
    Unknown,
}

/// Basic JSON response container
#[derive(Serialize)]
struct Info {
    name: String,
    value: String,
}

/// Main gateway to request the api routes
#[get("/api/{api_route}")]
pub async fn api(api_route: web::Path<String>) -> actix_web::Result<impl Responder> {

    let api_route_str = api_route.to_lowercase();

    let route: ApiRoute = match api_route_str.as_str() {
        "database_exists" => ApiRoute::DatabaseExists,
        _ => ApiRoute::Unknown,
    };

    match route {
        ApiRoute::DatabaseExists => {
            // Get the json response for checking the file exists on the system
            // Currently running the assumption that the only database is a local db

            return Ok(database_exists())
        },
        ApiRoute::Unknown => {
            return Ok(unknown_request())
        }
    }

}


// Non-HTTP functions called from main API


fn database_exists() -> web::Json<Info> {
    let exists = local::sqlite_db_exists();

    web::Json(
        Info{
            name: "database_exists".to_string(),
            value: exists.to_string()
            }
        )
}

fn unknown_request() -> web::Json<Info> {
    web::Json(
        Info {
            name: "error".to_string(),
            value: "unknown request".to_string()
        }
    )
}