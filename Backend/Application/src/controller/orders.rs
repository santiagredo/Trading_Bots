use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::structs::request::OrderRequest;

use crate::{handler::Orders, utils::error_response};

#[post("")]
pub async fn insert_order(web::Json(order): web::Json<OrderRequest>) -> impl Responder {
    match Orders::new(order).insert_order().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("")]
pub async fn select_order(order: web::Query<OrderRequest>) -> impl Responder {
    match Orders::new(order.into_inner()).select_order().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("")]
pub async fn update_order(web::Json(order): web::Json<OrderRequest>) -> impl Responder {
    match Orders::new(order).update_order().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
