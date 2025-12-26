use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, TaskRequest};

use crate::{handler::Tasks, utils::error_response};

// db
#[get("/{env}/all")]
pub async fn select_tasks(
    env: web::Path<Environments>,
    task: web::Query<TaskRequest>,
) -> impl Responder {
    match Tasks::new(task.into_inner())
        .with_env(env.into_inner())
        .select_tasks()
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
    match Tasks::new(task)
        .with_env(env.into_inner())
        .update_task()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// cache
#[get("/{env}/memory/all")]
pub async fn select_active_tasks(env: web::Path<Environments>) -> impl Responder {
    let result = Tasks::default()
        .with_env(env.into_inner())
        .get_active_tasks()
        .await;

    HttpResponse::Ok().json(result)
}

#[post("/{env}/memory/start")]
pub async fn start_active_tasks(env: web::Path<Environments>) -> impl Responder {
    match Tasks::default()
        .with_env(env.into_inner())
        .start_active_tasks()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[post("/{env}/memory/stop")]
pub async fn stop_active_tasks(env: web::Path<Environments>) -> impl Responder {
    Tasks::default()
        .with_env(env.into_inner())
        .stop_active_tasks()
        .await;

    HttpResponse::Ok().finish()
}
