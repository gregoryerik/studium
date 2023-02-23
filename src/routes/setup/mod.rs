/*

    Where all of the routes will go for API calls related to setup

*/

use actix_web::{get, post, HttpResponse, Responder, web, HttpRequest};
use askama::Template;
use serde::{Serialize, Deserialize};

use crate::routes::templates;
use crate::database::local;

use self::verify::Codes;

mod verify;


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

#[derive(Serialize, Deserialize, Debug)]
pub struct SetupData {
    name_input: String,
    email_input: String,
    sql_path_input: String
} 

#[derive(Serialize, Deserialize)]
pub struct SetupDataContainer {
    data: SetupData
}


#[get("/setup")]
pub async fn setup() -> actix_web::Result<HttpResponse> {
    let setup_template = templates::SetupTemplate {}.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(setup_template))
}


#[post("/setup")]
async fn post_setup(_req: HttpRequest, params: web::Form<SetupData>) -> impl Responder {
    // Creating the safe SetupData

    let mut safe_data = SetupData{
        name_input: params.name_input.clone(), // this instead of string new because then dont need match arm to write
        email_input: String::new(),
        sql_path_input: String::new()
    };

    // verifying the values

    //Check name
    let vf_name = verify::verify_name(safe_data.name_input.clone());

    match vf_name {
        Codes::MODIFY(val) => {
            // needs to be updated with the safe version
            safe_data.name_input = val;
        },
        _ => {}
    }

    // Check email. Only allow the modified version
    let vf_email = verify::verify_email(params.email_input.clone());

    match vf_email {
        Codes::MODIFY(val) => {
            safe_data.email_input = val;
        },
        Codes::REJECT => {
            // No need to go any further. The email is being rejected. Return back to verify page
            return HttpResponse::NotAcceptable().content_type("text/javascript").body("Error with email")
        },
        _ => {}
    }

    let vf_path = verify::verify_sql_path(params.sql_path_input.clone());

    match vf_path {
        Codes::ALLOW => {
            safe_data.sql_path_input = params.sql_path_input.clone();
        },
        Codes::MODIFY(val) => {
            safe_data.sql_path_input = val;
        }
        _ => {
            return HttpResponse::NotAcceptable().content_type("text/javascript").body("Error with path")
        }
    }

    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(format!("Data returned {:#?}", safe_data))

}

// DBT -> Database test
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