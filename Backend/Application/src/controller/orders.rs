use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::entities::orders::Model;

use crate::{
    types::Orders,
    utils::{Core, OutcomeError},
};

#[post("")]
pub async fn insert_order(web::Json(order): web::Json<Model>) -> impl Responder {
    match Orders::<Core>::insert_order(order).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/{id}")]
pub async fn select_order(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    match Orders::<Core>::select_order(id).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[patch("")]
pub async fn update_order(web::Json(order): web::Json<Model>) -> impl Responder {
    match Orders::<Core>::update_order(order).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}
