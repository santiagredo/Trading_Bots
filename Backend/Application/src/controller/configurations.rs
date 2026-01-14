use actix_web::{get, HttpResponse, Responder};

use crate::handler::Configurations;

#[get("")]
pub async fn select_configuration() -> impl Responder {
    let configuration = Configurations::default().select_configuration().await;
    HttpResponse::Ok().json(configuration)
}
