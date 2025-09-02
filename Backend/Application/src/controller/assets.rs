use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::AssetRequest;

use crate::{handler::Assets, utils::error_response};

#[post("")]
pub async fn insert_asset(web::Json(asset): web::Json<AssetRequest>) -> impl Responder {
    match Assets::new(asset).insert_asset().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("")]
pub async fn select_asset(asset: web::Query<AssetRequest>) -> impl Responder {
    match Assets::new(asset.into_inner()).select_asset().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/all")]
pub async fn select_assets(asset: web::Query<AssetRequest>) -> impl Responder {
    match Assets::new(asset.into_inner()).select_assets().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/memory")]
pub async fn get_assets() -> impl Responder {
    let val = Assets::get_active_assets().await;
    HttpResponse::Ok().json(val)
}

#[patch("")]
pub async fn update_asset(web::Json(asset): web::Json<AssetRequest>) -> impl Responder {
    match Assets::new(asset).update_asset().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("")]
pub async fn delete_asset(web::Json(asset): web::Json<AssetRequest>) -> impl Responder {
    match Assets::new(asset).delete_asset().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
