use actix_web::{get, post, web, HttpResponse, Responder};
use models::structs::{Environments, LedgerRequest, QueryOptions};

use crate::{handler::Ledgers, utils::error_response};

#[post("/{env}")]
pub async fn insert_ledger(
    env: web::Path<Environments>,
    web::Json(ledger): web::Json<LedgerRequest>,
) -> impl Responder {
    match Ledgers::new(ledger)
        .with_env(env.into_inner())
        .insert_ledger()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_ledger(
    env: web::Path<Environments>,
    query: web::Query<LedgerRequest>,
) -> impl Responder {
    match Ledgers::new(query.into_inner())
        .with_env(env.into_inner())
        .select_ledger()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_ledgers(
    env: web::Path<Environments>,
    ledger: web::Query<LedgerRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    match Ledgers::new(ledger.into_inner())
        .with_env(env.into_inner())
        .select_ledgers(Some(query.into_inner()))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
