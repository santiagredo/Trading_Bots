use actix_web::{get, HttpResponse, Responder};

use crate::{handler::HealthCheck, utils::error_response};

#[get("")]
pub async fn select_health_check() -> impl Responder {
    match HealthCheck::select_health_check(false).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
