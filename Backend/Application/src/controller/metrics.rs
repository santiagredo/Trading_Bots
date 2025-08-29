use actix_web::{get, HttpResponse, Responder};

use crate::types::Metrics;

#[get("")]
pub async fn select_metrics() -> impl Responder {
    let metrics = Metrics::get_active_metrics().await;

    HttpResponse::Ok().json(metrics)
}
