use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, OrderRequest};

use crate::{handler::Orders, utils::error_response};

#[post("/{env}")]
pub async fn insert_order(
    env: web::Path<Environments>,
    web::Json(order): web::Json<OrderRequest>,
) -> impl Responder {
    match Orders::new(order)
        .with_env(env.into_inner())
        .insert_order()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_order(
    env: web::Path<Environments>,
    query: web::Query<OrderRequest>,
) -> impl Responder {
    match Orders::new(query.into_inner())
        .with_env(env.into_inner())
        .select_order()
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
    match Orders::new(order)
        .with_env(env.into_inner())
        .update_order()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
