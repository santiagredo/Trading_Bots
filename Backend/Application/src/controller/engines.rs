use actix_web::{get, post, HttpResponse, Responder};

use crate::handler::Engines;

#[get("")]
pub async fn get_engine_status() -> impl Responder {
    let engine = Engines::default().get_engine_status().await;
    HttpResponse::Ok().json(engine)
}

#[post("/shutdown")]
pub async fn shutdown_engine() -> impl Responder {
    Engines::default().stop_engine().await;
    HttpResponse::Ok().json("Shutting down")
}
