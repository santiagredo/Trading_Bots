use actix_web::{post, web, HttpResponse, Responder};
use models::structs::Ticker;

use crate::types::Executor;

#[post("")]
pub async fn insert_backtests(web::Json(data): web::Json<Vec<Ticker>>) -> impl Responder {
    // match Assets::<Core>::insert_asset(asset).await {
    //     Ok(val) => HttpResponse::Ok().json(val),
    //     Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
    //     Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    // }

    let results = Executor::run_backtest(data).await;

    HttpResponse::Ok().json(results)
}
