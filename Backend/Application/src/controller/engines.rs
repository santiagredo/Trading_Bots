use actix_web::{get, post, HttpResponse, Responder};

use crate::{handler::Engines, utils::error_response};

#[get("")]
pub async fn get_engine_status() -> impl Responder {
    let engine = Engines::blank().get_engine_cache().await;
    HttpResponse::Ok().json(engine)
}

#[post("/shutdown")]
pub async fn shutdown_engine() -> impl Responder {
    match Engines::blank().stop_engine().await {
        Ok(_) => HttpResponse::Ok().json("Shutting down"),
        Err(err) => error_response(err),
    }
}
