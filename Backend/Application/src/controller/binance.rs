use actix_web::{get, web, HttpResponse, Responder};
use models::structs::Environments;

use crate::{handler::Binance, utils::error_response};

#[get("/{env}")]
pub async fn get_account(env: web::Path<Environments>) -> impl Responder {
    match Binance::default()
        .with_env(env.into_inner())
        .get_account_manually()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
