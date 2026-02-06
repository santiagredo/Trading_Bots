use actix_web::{get, patch, web, HttpResponse, Responder};
use models::structs::{Environments, IntegrationSettingRequest, QueryOptions};

use crate::{
    handler::IntegrationsSettings,
    utils::{error_response, DbRepo, EntityCache},
};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[get("/{env}")]
pub async fn select_integration_setting(
    env: web::Path<Environments>,
    integration_setting: web::Query<IntegrationSettingRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = IntegrationsSettings::new(repo);

    match service.select(integration_setting.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_integrations_settings(
    env: web::Path<Environments>,
    integration_setting: web::Query<IntegrationSettingRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = IntegrationsSettings::new(repo);

    match service
        .select_many(integration_setting.into_inner(), Some(query.into_inner()))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("/{env}")]
pub async fn update_integration_setting(
    env: web::Path<Environments>,
    web::Json(integration_setting): web::Json<IntegrationSettingRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = IntegrationsSettings::new(repo);

    match service.update(integration_setting).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// ============================================
// CACHE OPERATIONS
// ============================================

#[get("/{env}/memory")]
pub async fn get_integration_setting(
    env: web::Path<Environments>,
    integration_setting: web::Query<IntegrationSettingRequest>,
) -> impl Responder {
    let setting_id = match integration_setting.id {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Integration setting ID is required"),
    };

    match IntegrationsSettings::blank()
        .get(env.into_inner(), setting_id)
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_integrations_settings(env: web::Path<Environments>) -> impl Responder {
    match IntegrationsSettings::blank()
        .get_all(env.into_inner())
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
