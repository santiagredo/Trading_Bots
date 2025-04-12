
use actix_web::{get, post, web, HttpResponse, Responder};
use models::entities::ledgers::Model;

use crate::{types::Ledgers, utils::{Core, OutcomeError}};

#[post("")]
pub async fn insert_ledger(web::Json(ledger): web::Json<Model>) -> impl Responder {
    match Ledgers::<Core>::insert_ledger(ledger).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/{id}")]
pub async fn select_ledger(path: web::Path<i32>) -> impl Responder {
    let id = path.into_inner();

    match Ledgers::<Core>::select_ledger(id).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/many")]
pub async fn select_ledgers() -> impl Responder {
    match Ledgers::<Core>::select_ledgers().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}