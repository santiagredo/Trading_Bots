use actix_web::{get, patch, web, HttpResponse, Responder};
use models::structs::{Environments, IntegrationRequest, QueryOptions};

use crate::{
    handler::Integrations,
    utils::{error_response, DbRepo, EntityCache},
};

#[get("/{env}/all")]
pub async fn select_integrations(
    env: web::Path<Environments>,
    integration: web::Query<IntegrationRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Integrations::new(repo);

    match service
        .select_many(integration.into_inner(), Some(query.into_inner()))
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
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Integrations::new(repo);

    match service.update(integration).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_integrations(env: web::Path<Environments>) -> impl Responder {
    let env = env.into_inner();

    let repo = match DbRepo::new(env.clone()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Integrations::new(repo);

    match service.get_all(env).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
