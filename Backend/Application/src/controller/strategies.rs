use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, StrategyRequest};

use crate::{handler::Strategies, utils::error_response};

// db
#[post("/{env}")]
pub async fn insert_strategy(
    env: web::Path<Environments>,
    web::Json(strategy): web::Json<StrategyRequest>,
) -> impl Responder {
    match Strategies::new(strategy)
        .with_env(env.into_inner())
        .insert_strategy()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_strategy(
    env: web::Path<Environments>,
    strategy: web::Query<StrategyRequest>,
) -> impl Responder {
    match Strategies::new(strategy.into_inner())
        .with_env(env.into_inner())
        .select_strategy()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_strategies(
    env: web::Path<Environments>,
    strategy: web::Query<StrategyRequest>,
) -> impl Responder {
    match Strategies::new(strategy.into_inner())
        .with_env(env.into_inner())
        .select_strategies()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("/{env}")]
pub async fn update_strategy(
    env: web::Path<Environments>,
    web::Json(strategy): web::Json<StrategyRequest>,
) -> impl Responder {
    match Strategies::new(strategy)
        .with_env(env.into_inner())
        .update_strategy()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_strategy(
    env: web::Path<Environments>,
    web::Json(strategy): web::Json<StrategyRequest>,
) -> impl Responder {
    match Strategies::new(strategy)
        .with_env(env.into_inner())
        .delete_strategy()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// cache
#[get("/{env}/memory")]
pub async fn get_active_strategy(
    env: web::Path<Environments>,
    strategy: web::Query<StrategyRequest>,
) -> impl Responder {
    let active_strategy = Strategies::new(strategy.into_inner())
        .with_env(env.into_inner())
        .get_active_strategy()
        .await;

    match active_strategy {
        None => HttpResponse::NotFound().finish(),
        Some(val) => HttpResponse::Ok().json(val),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_active_strategies(
    env: web::Path<Environments>,
    strategy: web::Query<StrategyRequest>,
) -> impl Responder {
    let active_strategies = Strategies::new(strategy.into_inner())
        .with_env(env.into_inner())
        .get_active_strategies()
        .await;

    match active_strategies {
        None => HttpResponse::NotFound().finish(),
        Some(val) => HttpResponse::Ok().json(val),
    }
}

#[post("/{env}/memory/start")]
pub async fn start_active_strategies(env: web::Path<Environments>) -> impl Responder {
    match Strategies::default()
        .with_env(env.into_inner())
        .start_active_strategies()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[post("/{env}/memory/stop")]
pub async fn stop_active_strategies(env: web::Path<Environments>) -> impl Responder {
    Strategies::default()
        .with_env(env.into_inner())
        .stop_active_strategies()
        .await;

    HttpResponse::Ok().finish()
}
