use actix_web::{get, HttpResponse, Responder};

use crate::{handler::Binance, utils::error_response};

#[get("")]
pub async fn get_account() -> impl Responder {
    match Binance::default().get_account().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
