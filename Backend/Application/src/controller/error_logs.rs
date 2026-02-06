use crate::{
    handler::ErrorLogs,
    utils::{error_response, DbRepo},
};
use actix_web::{delete, get, web, HttpResponse, Responder};
use models::structs::{Environments, ErrorLogRequest, QueryOptions};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[get("/{env}")]
pub async fn select_error_log(
    env: web::Path<Environments>,
    error_log: web::Query<ErrorLogRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = ErrorLogs::new(repo);

    match service.select(error_log.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_error_logs(
    env: web::Path<Environments>,
    error_log: web::Query<ErrorLogRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = ErrorLogs::new(repo);

    match service
        .select_many(error_log.into_inner(), Some(query.into_inner()))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_error_log(
    env: web::Path<Environments>,
    web::Json(error_log): web::Json<ErrorLogRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = ErrorLogs::new(repo);

    match service.delete(error_log).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
