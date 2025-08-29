use actix_web::{get, web, HttpResponse, Responder};
use models::structs::StrategyRequest;

use crate::{types::StrategiesOverview, utils::error_response};

#[get("")]
pub async fn select_strategies_overview(query: web::Query<StrategyRequest>) -> impl Responder {
    let filter = query.into_inner();

    match StrategiesOverview::select_strategies_overview(filter).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
