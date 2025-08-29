use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::structs::PairRequest;

use crate::{types::Pairs, utils::error_response};

#[post("")]
pub async fn insert_pair(web::Json(pair): web::Json<PairRequest>) -> impl Responder {
    match Pairs::new(pair).insert_pair().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("")]
pub async fn select_pair(pair: web::Query<PairRequest>) -> impl Responder {
    match Pairs::new(pair.into_inner()).select_pair().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("")]
pub async fn update_pair(web::Json(pair): web::Json<PairRequest>) -> impl Responder {
    match Pairs::new(pair).update_pair().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
