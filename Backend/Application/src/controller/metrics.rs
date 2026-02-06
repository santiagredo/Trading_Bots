use actix_web::{get, web, HttpResponse, Responder};
use models::structs::{Environments, MetricRequest, QueryOptions};

use crate::{
    handler::Metrics,
    utils::{error_response, DbRepo},
};

#[get("/{env}")]
pub async fn select_metrics(
    env: web::Path<Environments>,
    metric: web::Query<MetricRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Metrics::new(repo);

    let metrics = service
        .select_many(metric.into_inner(), Some(query.into_inner()))
        .await;

    match metrics {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/memory")]
pub async fn get_metric(
    env: web::Path<Environments>,
    metric: web::Query<MetricRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Metrics::new(repo);

    let metrics = service.select(metric.into_inner()).await;

    match metrics {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
