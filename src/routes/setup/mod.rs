/*

    Where all of the routes will go for API calls related to setup

*/

use actix_web::{get, HttpResponse, Responder, web};
use askama::Template;
use serde::Serialize;

use crate::routes::templates;
use crate::database::local;


#[derive(Serialize)]
struct DatabaseTestReply {
    test_type: TestType,
    status: local::ConnectionStatus,
}

#[derive(Serialize)]
enum TestType {
    Local,
    Remote,
    RE, // Read Error
}


#[get("/setup")]
pub async fn setup() -> actix_web::Result<HttpResponse> {
    let setup_template = templates::SetupTemplate {}.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(setup_template))
}


//DBT -> Database test
#[get("/setup/dbt/{test}")] 
pub async fn database_setup(test: web::Path<String>) -> actix_web::Result<impl Responder> {

    let test_type: TestType = match test.as_str() {
        "local" => TestType::Local,
        "remote" => TestType::Remote,
        _ => TestType::RE,
    };

    

    let reply = DatabaseTestReply {
        test_type: test_type,
        status: local::test_connection()
    };

    Ok(web::Json(reply))
    
}