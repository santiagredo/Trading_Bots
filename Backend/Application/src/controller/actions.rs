use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use models::structs::{ActionRequest, Environments};

use crate::{handler::Actions, utils::error_response};

// db
#[post("/{env}")]
pub async fn insert_action(
    env: web::Path<Environments>,
    web::Json(action): web::Json<ActionRequest>,
) -> impl Responder {
    match Actions::new(action)
        .with_env(env.into_inner())
        .insert_action()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_action(
    env: web::Path<Environments>,
    action: web::Query<ActionRequest>,
) -> impl Responder {
    match Actions::new(action.into_inner())
        .with_env(env.into_inner())
        .select_action()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_actions(
    env: web::Path<Environments>,
    action: web::Query<ActionRequest>,
) -> impl Responder {
    match Actions::new(action.into_inner())
        .with_env(env.into_inner())
        .select_actions()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[put("/{env}")]
pub async fn update_action(
    env: web::Path<Environments>,
    web::Json(action): web::Json<ActionRequest>,
) -> impl Responder {
    match Actions::new(action)
        .with_env(env.into_inner())
        .update_action()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_action(
    env: web::Path<Environments>,
    web::Json(action): web::Json<ActionRequest>,
) -> impl Responder {
    match Actions::new(action)
        .with_env(env.into_inner())
        .delete_action()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// cache
#[get("/{env}/memory")]
pub async fn get_active_action(
    env: web::Path<Environments>,
    action: web::Query<ActionRequest>,
) -> impl Responder {
    match Actions::new(action.into_inner())
        .with_env(env.into_inner())
        .get_active_action()
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_active_actions(env: web::Path<Environments>) -> impl Responder {
    match Actions::default()
        .with_env(env.into_inner())
        .get_active_actions()
        .await
    {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[post("/{env}/memory/start")]
pub async fn start_active_actions(env: web::Path<Environments>) -> impl Responder {
    match Actions::default()
        .with_env(env.into_inner())
        .start_active_actions()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[post("/{env}/memory/stop")]
pub async fn stop_active_actions(env: web::Path<Environments>) -> impl Responder {
    Actions::default()
        .with_env(env.into_inner())
        .stop_active_actions()
        .await;

    HttpResponse::Ok().finish()
}
