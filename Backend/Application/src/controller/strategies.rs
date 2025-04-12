use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::entities::strategies::Model;

use crate::{
    types::Strategies,
    utils::{Core, OutcomeError},
};

#[post("")]
pub async fn insert_strategy(web::Json(strategy): web::Json<Model>) -> impl Responder {
    match Strategies::<Core>::insert_strategy(strategy).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/{id}")]
pub async fn select_strategy(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    match Strategies::<Core>::select_strategy(id).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/many")]
pub async fn select_active_strategies() -> impl Responder {
    match Strategies::<Core>::select_active_strategies().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[patch("")]
pub async fn update_strategy(web::Json(strategy): web::Json<Model>) -> impl Responder {
    match Strategies::<Core>::update_strategy(strategy).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[delete("")]
pub async fn delete_strategy(web::Json(strategy): web::Json<Model>) -> impl Responder {
    match Strategies::<Core>::delete_strategy(strategy).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}
