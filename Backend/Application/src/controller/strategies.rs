use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, QueryOptions, StrategyRequest};

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
    query: web::Query<QueryOptions>,
) -> impl Responder {
    match Strategies::new(strategy.into_inner())
        .with_env(env.into_inner())
        .select_strategies(Some(query.into_inner()))
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
pub async fn get_strategy(
    env: web::Path<Environments>,
    strategy: web::Query<StrategyRequest>,
) -> impl Responder {
    let active_strategy = Strategies::new(strategy.into_inner())
        .with_env(env.into_inner())
        .get_strategy()
        .await;

    match active_strategy {
        None => HttpResponse::NotFound().finish(),
        Some(val) => HttpResponse::Ok().json(val),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_strategies(
    env: web::Path<Environments>,
    strategy: web::Query<StrategyRequest>,
) -> impl Responder {
    let active_strategies = Strategies::new(strategy.into_inner())
        .with_env(env.into_inner())
        .get_strategies()
        .await;

    match active_strategies {
        None => HttpResponse::NotFound().finish(),
        Some(val) => HttpResponse::Ok().json(val),
    }
}
