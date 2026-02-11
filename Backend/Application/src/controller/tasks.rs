use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, QueryOptions, TaskRequest};

use crate::{
    handler::{Cancellations, Tasks},
    utils::{error_response, DbRepo, EntityCache, RepoFactory, Response},
};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[get("/{env}/all")]
pub async fn select_tasks(
    env: web::Path<Environments>,
    task: web::Query<TaskRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Tasks::new(repo);

    match service
        .select(task.into_inner(), Some(query.into_inner()))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("/{env}")]
pub async fn update_task(
    env: web::Path<Environments>,
    web::Json(task): web::Json<TaskRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Tasks::new(repo);

    match service.update(task).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// ============================================
// CACHE OPERATIONS
// ============================================

#[get("/{env}/memory")]
pub async fn get_task(
    env: web::Path<Environments>,
    task: web::Query<TaskRequest>,
) -> impl Responder {
    let task_id = match task.id {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Task ID is required"),
    };

    match Tasks::blank().get(env.into_inner(), task_id).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_tasks(env: web::Path<Environments>) -> impl Responder {
    match Tasks::blank().get_all(env.into_inner()).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{env}/memory/start")]
pub async fn start_tasks(env: web::Path<Environments>) -> impl Responder {
    let env = env.into_inner();

    let factory = match RepoFactory::db(env).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let repo = factory.repo();

    let runtime_token = match Cancellations::new().get_runtime_token(env).await {
        None => return error_response(Response::not_found("Runtime token".to_string())),
        Some(val) => val,
    };

    match Tasks::new(repo).start(factory, env, &runtime_token).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => error_response(err),
    }
}

#[post("/{env}/memory/stop")]
pub async fn stop_tasks(env: web::Path<Environments>) -> impl Responder {
    match Tasks::blank().stop(env.into_inner()).await {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(err) => error_response(err),
    }
}
