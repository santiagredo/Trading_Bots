use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, PairRequest};

use crate::{handler::Pairs, utils::error_response};

// db
#[post("/{env}")]
pub async fn insert_pair(
    env: web::Path<Environments>,
    web::Json(pair): web::Json<PairRequest>,
) -> impl Responder {
    match Pairs::new(pair)
        .with_env(env.into_inner())
        .insert_pair()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_pair(
    env: web::Path<Environments>,
    query: web::Query<PairRequest>,
) -> impl Responder {
    match Pairs::new(query.into_inner())
        .with_env(env.into_inner())
        .select_pair()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_pairs(
    env: web::Path<Environments>,
    pair: web::Query<PairRequest>,
) -> impl Responder {
    match Pairs::new(pair.into_inner())
        .with_env(env.into_inner())
        .select_pairs()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("/{env}")]
pub async fn update_pair(
    env: web::Path<Environments>,
    web::Json(pair): web::Json<PairRequest>,
) -> impl Responder {
    match Pairs::new(pair)
        .with_env(env.into_inner())
        .update_pair()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// cache
#[get("/{env}/memory")]
pub async fn get_pair(
    env: web::Path<Environments>,
    pair: web::Query<PairRequest>,
) -> impl Responder {
    match Pairs::new(pair.into_inner())
        .with_env(env.into_inner())
        .get_pair()
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_pairs(env: web::Path<Environments>) -> impl Responder {
    match Pairs::default()
        .with_env(env.into_inner())
        .get_pairs()
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
