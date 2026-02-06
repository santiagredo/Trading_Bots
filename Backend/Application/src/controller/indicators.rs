use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, IndicatorRequest, QueryOptions};

use crate::{
    handler::{Indicators, SubscribedIndicators},
    utils::{error_response, DbRepo, EntityCache},
};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[post("/{env}")]
pub async fn insert_indicator(
    env: web::Path<Environments>,
    web::Json(indicator): web::Json<IndicatorRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Indicators::new(repo);

    match service.insert(indicator).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_indicator(
    env: web::Path<Environments>,
    query: web::Query<IndicatorRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Indicators::new(repo);

    match service.select(query.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_indicators(
    env: web::Path<Environments>,
    indicator: web::Query<IndicatorRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Indicators::new(repo);

    match service
        .select_many(indicator.into_inner(), Some(query.into_inner()))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("/{env}")]
pub async fn update_indicator(
    env: web::Path<Environments>,
    web::Json(indicator): web::Json<IndicatorRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Indicators::new(repo);

    match service.update(indicator).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_indicator(
    env: web::Path<Environments>,
    web::Json(indicator): web::Json<IndicatorRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Indicators::new(repo);

    match service.delete(indicator).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// ============================================
// CACHE OPERATIONS
// ============================================

#[get("/{env}/memory")]
pub async fn get_indicator(
    env: web::Path<Environments>,
    query: web::Query<IndicatorRequest>,
) -> impl Responder {
    let indicator_id = match query.id {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Indicator ID is required"),
    };

    match Indicators::blank()
        .get(env.into_inner(), indicator_id)
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_indicators(env: web::Path<Environments>) -> impl Responder {
    match Indicators::blank().get_all(env.into_inner()).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/subscribed_indicators/all")]
pub async fn get_subscribed_indicators(env: web::Path<Environments>) -> impl Responder {
    match SubscribedIndicators::blank()
        .get_all(env.into_inner())
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
