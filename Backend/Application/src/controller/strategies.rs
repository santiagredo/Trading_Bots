use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::StrategyRequest;

use crate::{handler::Strategies, utils::error_response};

#[post("")]
pub async fn insert_strategy(web::Json(strategy): web::Json<StrategyRequest>) -> impl Responder {
    match Strategies::new(strategy).insert_strategy().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("")]
pub async fn select_strategy(strategy: web::Query<StrategyRequest>) -> impl Responder {
    match Strategies::new(strategy.into_inner())
        .select_strategy()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/all")]
pub async fn select_strategies(strategy: web::Query<StrategyRequest>) -> impl Responder {
    match Strategies::new(strategy.into_inner())
        .select_strategies()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("")]
pub async fn update_strategy(web::Json(strategy): web::Json<StrategyRequest>) -> impl Responder {
    match Strategies::new(strategy).update_strategy().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("")]
pub async fn delete_strategy(web::Json(strategy): web::Json<StrategyRequest>) -> impl Responder {
    match Strategies::new(strategy).delete_strategy().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
