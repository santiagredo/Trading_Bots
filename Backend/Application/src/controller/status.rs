use actix_web::{get, HttpResponse, Responder};

use crate::{types::OrderStatus, utils::error_response};

#[get("")]
pub async fn select_status() -> impl Responder {
    match OrderStatus::default().select_status().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
