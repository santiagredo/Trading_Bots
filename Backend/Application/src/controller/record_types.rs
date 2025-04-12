use actix_web::{get, HttpResponse, Responder};

use crate::{types::RecordTypes, utils::{Core, OutcomeError}};


#[get("/many")]
pub async fn select_record_types() -> impl Responder {
    match RecordTypes::<Core>::select_record_types().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(OutcomeError::Failure(fail)) => HttpResponse::BadRequest().json(fail),
        Err(OutcomeError::Error(err)) => HttpResponse::InternalServerError().json(err),
    }
}