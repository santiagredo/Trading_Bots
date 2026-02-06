use actix_web::{get, HttpResponse, Responder};

use crate::handler::Runtimes;

#[get("")]
pub async fn get_runtimes_status() -> impl Responder {
    let runtimes = Runtimes::new().get_runtimes_status().await;
    HttpResponse::Ok().json(runtimes)
}
