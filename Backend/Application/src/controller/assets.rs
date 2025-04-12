use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::entities::assets::{self, Model};

use crate::{
    types::Assets,
    utils::{Core, OutcomeError},
};

#[post("")]
pub async fn insert_asset(web::Json(asset): web::Json<Model>) -> impl Responder {
    match Assets::<Core>::insert_asset(asset).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/{id}")]
pub async fn select_asset(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    let model = assets::Model {
        id,
        ..Default::default()
    };

    match Assets::<Core>::select_asset(model).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/many")]
pub async fn select_assets() -> impl Responder {
    match Assets::<Core>::select_assets().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[patch("")]
pub async fn update_asset(web::Json(asset): web::Json<Model>) -> impl Responder {
    match Assets::<Core>::update_asset(asset).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[delete("")]
pub async fn delete_asset(web::Json(asset): web::Json<Model>) -> impl Responder {
    match Assets::<Core>::delete_asset(asset).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}
