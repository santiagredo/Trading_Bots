use actix_web::{get, patch, web, HttpResponse, Responder};
use models::structs::{Environments, IntegrationSettingRequest, QueryOptions};

use crate::{handler::IntegrationsSettings, utils::error_response};

#[get("/{env}/all")]
pub async fn select_integrations_settings(
    env: web::Path<Environments>,
    integration_setting: web::Query<IntegrationSettingRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    match IntegrationsSettings::new(integration_setting.into_inner())
        .with_env(env.into_inner())
        .select_integrations_settings(Some(query.into_inner()))
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
    match IntegrationsSettings::new(integration_setting)
        .with_env(env.into_inner())
        .update_integration_setting()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// cache
#[get("/{env}/memory/all")]
pub async fn get_integrations_settings(env: web::Path<Environments>) -> impl Responder {
    match IntegrationsSettings::default()
        .with_env(env.into_inner())
        .get_integrations_settings()
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
