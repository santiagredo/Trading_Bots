use crate::{handler::IntegrationLogs, utils::{DbRepo, error_response}};
use actix_web::{delete, get, web, HttpResponse, Responder};
use models::structs::{Environments, IntegrationLogRequest, QueryOptions};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[get("/{env}")]
pub async fn select_integration_log(
    env: web::Path<Environments>,
    integration_log: web::Query<IntegrationLogRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = IntegrationLogs::new(repo);

    match service.select(integration_log.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_integration_logs(
    env: web::Path<Environments>,
    integration_log: web::Query<IntegrationLogRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = IntegrationLogs::new(repo);

    match service
        .select_many(integration_log.into_inner(), Some(query.into_inner()))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_integration_log(
    env: web::Path<Environments>,
    web::Json(integration_log): web::Json<IntegrationLogRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = IntegrationLogs::new(repo);

    match service.delete(integration_log).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
