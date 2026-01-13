use actix_web::{get, web, HttpResponse, Responder};
use models::structs::Environments;

use crate::{handler::Metrics, utils::error_response};

#[get("/{env}")]
pub async fn select_metrics(env: web::Path<Environments>) -> impl Responder {
    let metrics = Metrics::default()
        .with_env(env.into_inner())
        .select_metrics()
        .await;

    match metrics {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/memory")]
pub async fn get_metric(env: web::Path<Environments>) -> impl Responder {
    let metrics = Metrics::default()
        .with_env(env.into_inner())
        .get_metric()
        .await;

    HttpResponse::Ok().json(metrics)
}
