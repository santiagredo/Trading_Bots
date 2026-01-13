use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::{AssetRequest, Environments};

use crate::{handler::Assets, utils::error_response};

// db
#[post("/{env}")]
pub async fn insert_asset(
    env: web::Path<Environments>,
    web::Json(asset): web::Json<AssetRequest>,
) -> impl Responder {
    match Assets::new(asset)
        .with_env(env.into_inner())
        .insert_asset()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_asset(
    env: web::Path<Environments>,
    asset: web::Query<AssetRequest>,
) -> impl Responder {
    match Assets::new(asset.into_inner())
        .with_env(env.into_inner())
        .select_asset()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_assets(
    env: web::Path<Environments>,
    asset: web::Query<AssetRequest>,
) -> impl Responder {
    match Assets::new(asset.into_inner())
        .with_env(env.into_inner())
        .select_assets()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("/{env}")]
pub async fn update_asset(
    env: web::Path<Environments>,
    web::Json(asset): web::Json<AssetRequest>,
) -> impl Responder {
    match Assets::new(asset)
        .with_env(env.into_inner())
        .update_asset()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_asset(
    env: web::Path<Environments>,
    web::Json(asset): web::Json<AssetRequest>,
) -> impl Responder {
    match Assets::new(asset)
        .with_env(env.into_inner())
        .delete_asset()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// cache
#[get("/{env}/memory")]
pub async fn get_asset(
    env: web::Path<Environments>,
    asset: web::Query<AssetRequest>,
) -> impl Responder {
    match Assets::new(asset.into_inner())
        .with_env(env.into_inner())
        .get_asset()
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_assets(env: web::Path<Environments>) -> impl Responder {
    match Assets::default()
        .with_env(env.into_inner())
        .get_assets()
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
