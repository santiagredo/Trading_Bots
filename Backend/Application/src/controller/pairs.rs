use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, PairRequest, QueryOptions};

use crate::{
    handler::Pairs,
    utils::{error_response, DbRepo, EntityCache},
};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[post("/{env}")]
pub async fn insert_pair(
    env: web::Path<Environments>,
    web::Json(pair): web::Json<PairRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Pairs::new(repo);

    match service.insert(pair).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_pair(
    env: web::Path<Environments>,
    pair: web::Query<PairRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Pairs::new(repo);

    match service.select(pair.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_pairs(
    env: web::Path<Environments>,
    pair: web::Query<PairRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Pairs::new(repo);

    match service
        .select_many(pair.into_inner(), Some(query.into_inner()))
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
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Pairs::new(repo);

    match service.update(pair).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// ============================================
// CACHE OPERATIONS
// ============================================

#[get("/{env}/memory")]
pub async fn get_pair(
    env: web::Path<Environments>,
    pair: web::Query<PairRequest>,
) -> impl Responder {
    let pair_id = match pair.id {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Pair ID is required"),
    };

    match Pairs::blank().get(env.into_inner(), pair_id).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_pairs(env: web::Path<Environments>) -> impl Responder {
    match Pairs::blank().get_all(env.into_inner()).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
