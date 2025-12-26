use actix_web::{get, web, HttpResponse, Responder};
use models::structs::{Environments, StrategyRequest};

use crate::{handler::StrategiesOverview, utils::error_response};

#[get("/{env}")]
pub async fn select_strategies_overview(
    env: web::Path<Environments>,
    query: web::Query<StrategyRequest>,
) -> impl Responder {
    match StrategiesOverview::select_strategy_overview(env.into_inner(), query.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
