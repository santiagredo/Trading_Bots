use actix_web::{get, patch, post, web, HttpResponse, Responder};
use models::structs::{Environments, QueryOptions, TaskRequest};

use crate::{
    handler::Tasks,
    utils::{error_response, RepoFactory},
};

// db
#[get("/{env}/all")]
pub async fn select_tasks(
    env: web::Path<Environments>,
    task: web::Query<TaskRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    match Tasks::new()
        .select_tasks(
            env.into_inner(),
            task.into_inner(),
            Some(query.into_inner()),
        )
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
    match Tasks::new().update_task(env.into_inner(), task).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// cache
#[get("/{env}/memory/all")]
pub async fn get_tasks(env: web::Path<Environments>) -> impl Responder {
    let result = Tasks::new().get_tasks(env.into_inner()).await;

    HttpResponse::Ok().json(result)
}

#[post("/{env}/memory/start")]
pub async fn start_tasks_manually(env: web::Path<Environments>) -> impl Responder {
    let env = env.into_inner();

    let factory = match RepoFactory::db(env).await {
        Err(err) => return error_response(err),
        Ok(val) => val,
    };

    match Tasks::new().start_tasks_manually(factory, env).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[post("/{env}/memory/stop")]
pub async fn stop_tasks(env: web::Path<Environments>) -> impl Responder {
    match Tasks::new().stop_tasks(env.into_inner()).await {
        Err(err) => error_response(err),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}
