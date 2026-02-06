use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, QueryOptions, StrategyRequest};

use crate::{
    handler::Strategies,
    utils::{error_response, DbRepo, EntityCache},
};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[post("/{env}")]
pub async fn insert_strategy(
    env: web::Path<Environments>,
    web::Json(strategy): web::Json<StrategyRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Strategies::new(repo);

    match service.insert(strategy).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_strategy(
    env: web::Path<Environments>,
    strategy: web::Query<StrategyRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Strategies::new(repo);

    match service.select(strategy.into_inner()).await {
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
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Strategies::new(repo);

    match service
        .select_many(strategy.into_inner(), Some(query.into_inner()))
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
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Strategies::new(repo);

    match service.update(strategy).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_strategy(
    env: web::Path<Environments>,
    web::Json(strategy): web::Json<StrategyRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Strategies::new(repo);

    match service.delete(strategy).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// ============================================
// CACHE OPERATIONS
// ============================================

#[get("/{env}/memory")]
pub async fn get_strategy(
    env: web::Path<Environments>,
    strategy: web::Query<StrategyRequest>,
) -> impl Responder {
    let strategy_id = match strategy.id {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Strategy ID is required"),
    };

    match Strategies::blank().get(env.into_inner(), strategy_id).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_strategies(env: web::Path<Environments>) -> impl Responder {
    match Strategies::blank().get_all(env.into_inner()).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
