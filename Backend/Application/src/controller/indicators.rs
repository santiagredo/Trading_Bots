use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, IndicatorRequest};

use crate::{
    handler::{Indicators, SubscribedIndicators},
    utils::error_response,
};

// db
#[post("/{env}")]
pub async fn insert_indicator(
    env: web::Path<Environments>,
    web::Json(indicator): web::Json<IndicatorRequest>,
) -> impl Responder {
    match Indicators::new(indicator)
        .with_env(env.into_inner())
        .insert_indicator()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_indicator(
    env: web::Path<Environments>,
    query: web::Query<IndicatorRequest>,
) -> impl Responder {
    match Indicators::new(query.into_inner())
        .with_env(env.into_inner())
        .select_indicator()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_indicators(
    env: web::Path<Environments>,
    query: web::Query<IndicatorRequest>,
) -> impl Responder {
    match Indicators::new(query.into_inner())
        .with_env(env.into_inner())
        .select_indicators()
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
    match Indicators::new(indicator)
        .with_env(env.into_inner())
        .update_indicator()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_indicator(
    env: web::Path<Environments>,
    web::Json(indicator): web::Json<IndicatorRequest>,
) -> impl Responder {
    match Indicators::new(indicator)
        .with_env(env.into_inner())
        .delete_indicator()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// cache
#[get("/{env}/memory")]
pub async fn get_active_indicator(
    env: web::Path<Environments>,
    query: web::Query<IndicatorRequest>,
) -> impl Responder {
    let active_indicator = Indicators::new(query.into_inner())
        .with_env(env.into_inner())
        .get_active_indicator()
        .await;

    match active_indicator {
        None => HttpResponse::NotFound().finish(),
        Some(val) => HttpResponse::Ok().json(val),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_active_indicators(env: web::Path<Environments>) -> impl Responder {
    let active_indicators = Indicators::default()
        .with_env(env.into_inner())
        .get_active_indicators()
        .await;

    match active_indicators {
        None => HttpResponse::NotFound().finish(),
        Some(val) => HttpResponse::Ok().json(val),
    }
}

#[get("/{env}/memory/subscribed_indicators/all")]
pub async fn get_subscribed_indicators(query: web::Path<Environments>) -> impl Responder {
    let request = SubscribedIndicators::new(query.into_inner());

    match request.get_active_subscribed_indicators().await {
        None => HttpResponse::NotFound().finish(),
        Some(val) => HttpResponse::Ok().json(val),
    }
}

#[post("/{env}/memory/start")]
pub async fn start_active_indicators(env: web::Path<Environments>) -> impl Responder {
    match Indicators::default()
        .with_env(env.into_inner())
        .start_active_indicators()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[post("/{env}/memory/stop")]
pub async fn stop_active_indicators(env: web::Path<Environments>) -> impl Responder {
    Indicators::default()
        .with_env(env.into_inner())
        .stop_active_indicators()
        .await;

    HttpResponse::Ok().finish()
}
