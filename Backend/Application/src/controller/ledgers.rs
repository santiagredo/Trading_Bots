use actix_web::{get, post, web, HttpResponse, Responder};
use models::structs::LedgerRequest;

use crate::{handler::Ledgers, utils::error_response};

#[post("")]
pub async fn insert_ledger(web::Json(ledger): web::Json<LedgerRequest>) -> impl Responder {
    match Ledgers::new(ledger).insert_ledger().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("")]
pub async fn select_ledger(query: web::Query<LedgerRequest>) -> impl Responder {
    match Ledgers::new(query.into_inner()).select_ledger().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/all")]
pub async fn select_ledgers(query: web::Query<LedgerRequest>) -> impl Responder {
    match Ledgers::new(query.into_inner()).select_ledgers().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
