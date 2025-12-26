use actix_web::{get, web, HttpResponse, Responder};
use models::structs::Environments;

use crate::handler::Metrics;

#[get("/{env}")]
pub async fn select_metrics(env: web::Path<Environments>) -> impl Responder {
    let metrics = Metrics::default()
        .with_env(env.into_inner())
        .get_active_metric()
        .await;

    HttpResponse::Ok().json(metrics)
}
