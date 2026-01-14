use actix_web::{get, patch, web, HttpResponse, Responder};
use models::structs::{Environments, IntegrationRequest};

use crate::{handler::Integrations, utils::error_response};

#[get("/{env}/all")]
pub async fn select_integrations(
    env: web::Path<Environments>,
    integration: web::Query<IntegrationRequest>,
) -> impl Responder {
    match Integrations::new(integration.into_inner())
        .with_env(env.into_inner())
        .select_integrations()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("/{env}")]
pub async fn update_integration(
    env: web::Path<Environments>,
    web::Json(integration): web::Json<IntegrationRequest>,
) -> impl Responder {
    match Integrations::new(integration)
        .with_env(env.into_inner())
        .update_integration()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// cache
#[get("/{env}/memory/all")]
pub async fn get_integrations(env: web::Path<Environments>) -> impl Responder {
    match Integrations::default()
        .with_env(env.into_inner())
        .get_integrations()
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
