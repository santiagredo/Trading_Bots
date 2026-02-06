use actix_web::{get, web, HttpResponse, Responder};
use models::structs::Environments;

use crate::{handler::OrderStatus, utils::error_response};

#[get("")]
pub async fn select_status(env: web::Path<Environments>) -> impl Responder {
    match OrderStatus::select(env.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
