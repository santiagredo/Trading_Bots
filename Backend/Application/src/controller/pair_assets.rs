use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::entities::pair_assets::{self, Model};

use crate::{
    types::PairAssets,
    utils::{Core, OutcomeError},
};

#[post("")]
pub async fn insert_pair_asset(web::Json(pair_asset): web::Json<Model>) -> impl Responder {
    match PairAssets::<Core>::insert_pair_asset(pair_asset).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/{id}")]
pub async fn select_pair_asset(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    let model = pair_assets::Model {
        id,
        ..Default::default()
    };

    match PairAssets::<Core>::select_pair_asset(model).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[patch("")]
pub async fn update_pair_asset(web::Json(pair_asset): web::Json<Model>) -> impl Responder {
    match PairAssets::<Core>::update_pair_asset(pair_asset).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}
