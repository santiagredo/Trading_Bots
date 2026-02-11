use crate::{handler::OrderStatus, utils::EntityCache};
use actix_web::{get, web, HttpResponse, Responder};
use models::structs::Environments;

#[get("")]
pub async fn select_status(env: web::Path<Environments>) -> impl Responder {
    match OrderStatus::blank().get_all(env.into_inner()).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
