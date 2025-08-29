use actix_web::{get, HttpResponse, Responder};

use crate::types::Binance;

#[get("")]
pub async fn get_account() -> impl Responder {
    let account = Binance::default().get_account().await;
    HttpResponse::Ok().json(account)
}
