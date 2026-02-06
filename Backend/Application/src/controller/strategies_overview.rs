use crate::{handler::StrategiesOverview, utils::error_response};
use actix_web::{get, web, HttpResponse, Responder};
use models::structs::{Environments, StrategyRequest};

#[get("/{env}")]
pub async fn select_strategies_overview(
    env: web::Path<Environments>,
    strategy: web::Query<StrategyRequest>,
) -> impl Responder {
    match StrategiesOverview::select(env.into_inner(), strategy.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
