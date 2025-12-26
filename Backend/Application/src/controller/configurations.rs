use actix_web::{get, post, web, HttpResponse, Responder};
use models::structs::ConfigurationRequest;

use crate::{handler::Configurations, utils::error_response};

#[post("")]
pub async fn insert_configuration(
    web::Json(configuration): web::Json<ConfigurationRequest>,
) -> impl Responder {
    match Configurations::new(configuration)
        .insert_configuration()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("")]
pub async fn select_configuration() -> impl Responder {
    let configuration = Configurations::default().select_configuration().await;
    HttpResponse::Ok().json(configuration)
}
