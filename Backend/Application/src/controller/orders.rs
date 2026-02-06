use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, OrderRequest, QueryOptions};

use crate::{
    handler::Orders,
    utils::{error_response, DbRepo},
};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[post("/{env}")]
pub async fn insert_order(
    env: web::Path<Environments>,
    web::Json(order): web::Json<OrderRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Orders::new(repo);

    match service.insert(order).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_order(
    env: web::Path<Environments>,
    order: web::Query<OrderRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Orders::new(repo);

    match service.select(order.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_orders(
    env: web::Path<Environments>,
    order: web::Query<OrderRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Orders::new(repo);

    match service
        .select_many(order.into_inner(), Some(query.into_inner()))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("/{env}")]
pub async fn update_order(
    env: web::Path<Environments>,
    web::Json(order): web::Json<OrderRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Orders::new(repo);

    match service.update(order).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
