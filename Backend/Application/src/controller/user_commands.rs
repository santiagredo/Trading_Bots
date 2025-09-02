use actix_web::{post, HttpResponse, Responder};

use crate::{handler::UserCommands, utils::error_response};

#[post("/start_everything")]
pub async fn start_everything() -> impl Responder {
    match UserCommands::start_everything().await {
        Err(err) => error_response(err),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}

#[post("/stop_everything")]
pub async fn stop_everything() -> impl Responder {
    UserCommands::stop_everything().await;
    HttpResponse::Ok().finish()
}

#[post("/refresh_everything")]
pub async fn refresh_everything() -> impl Responder {
    match UserCommands::refresh_everything().await {
        Err(err) => error_response(err),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}
