use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::{AssetRequest, Environments, QueryOptions};

use crate::{
    handler::Assets,
    utils::{error_response, DbRepo, EntityCache},
};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[post("/{env}")]
pub async fn insert_asset(
    env: web::Path<Environments>,
    web::Json(asset): web::Json<AssetRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Assets::new(repo);

    match service.insert(asset).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_asset(
    env: web::Path<Environments>,
    asset: web::Query<AssetRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Assets::new(repo);

    match service.select(asset.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_assets(
    env: web::Path<Environments>,
    asset: web::Query<AssetRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Assets::new(repo);

    match service
        .select_many(asset.into_inner(), Some(query.into_inner()))
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
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Assets::new(repo);

    match service.update(asset).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_asset(
    env: web::Path<Environments>,
    web::Json(asset): web::Json<AssetRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Assets::new(repo);

    match service.delete(asset).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// ============================================
// CACHE OPERATIONS
// ============================================

#[get("/{env}/memory")]
pub async fn get_asset(
    env: web::Path<Environments>,
    asset: web::Query<AssetRequest>,
) -> impl Responder {
    let asset_id = match asset.id {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Asset ID is required"),
    };

    match Assets::blank().get(env.into_inner(), asset_id).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_assets(env: web::Path<Environments>) -> impl Responder {
    match Assets::blank().get_all(env.into_inner()).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
