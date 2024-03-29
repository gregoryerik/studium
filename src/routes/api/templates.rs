/*
//
//  All of the responses via an API that are templated but not full pages
//
 */

use actix_web::{get, HttpResponse, web};
use askama::Template;

use crate::routes::template_models as tmpl;
 
#[get("/template/card/{name}")]
pub async fn card(info: web::Path<String>) -> actix_web::Result<HttpResponse> {

    let t= info.into_inner();
    let template = tmpl::SubjectFrameTemplate {
        title: Some(t.as_str()),
        subtitle: None,
        deck_size: None,
        to_update: None,
    }.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(template))
}

#[get("/template/ecard")]
pub async fn e_card() -> actix_web::Result<HttpResponse> {
    // returns an empty card template

    let template = tmpl::SubjectFrameTemplate{
        title: None,
        subtitle: None,
        deck_size: None,
        to_update: None,
    }.render().unwrap();

    Ok(HttpResponse::Ok().content_type("text/html").body(template))

}