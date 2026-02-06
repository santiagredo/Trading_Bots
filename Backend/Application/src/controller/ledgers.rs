use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, LedgerRequest, QueryOptions};

use crate::{
    handler::Ledgers,
    utils::{error_response, DbRepo},
};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[post("/{env}")]
pub async fn insert_ledger(
    env: web::Path<Environments>,
    web::Json(ledger): web::Json<LedgerRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Ledgers::new(repo);

    match service.insert(ledger).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_ledger(
    env: web::Path<Environments>,
    ledger: web::Query<LedgerRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Ledgers::new(repo);

    match service.select(ledger.into_inner()).await {
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
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Ledgers::new(repo);

    match service
        .select_many(ledger.into_inner(), Some(query.into_inner()))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("/{env}")]
pub async fn update_ledger(
    env: web::Path<Environments>,
    web::Json(ledger): web::Json<LedgerRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Ledgers::new(repo);

    match service.update(ledger).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_ledger(
    env: web::Path<Environments>,
    web::Json(ledger): web::Json<LedgerRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Ledgers::new(repo);

    match service.delete(ledger).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
