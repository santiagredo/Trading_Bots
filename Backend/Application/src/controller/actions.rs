use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use models::structs::ActionRequest;

use crate::{types::Actions, utils::error_response};

#[post("")]
pub async fn insert_action(web::Json(action): web::Json<ActionRequest>) -> impl Responder {
    match Actions::new(action).insert_action().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("")]
pub async fn select_action(action: web::Query<ActionRequest>) -> impl Responder {
    match Actions::new(action.into_inner()).insert_action().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/all")]
pub async fn select_actions(action: web::Query<ActionRequest>) -> impl Responder {
    match Actions::new(action.into_inner()).select_actions().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[put("")]
pub async fn update_action(web::Json(action): web::Json<ActionRequest>) -> impl Responder {
    match Actions::new(action).update_action().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("")]
pub async fn delete_action(web::Json(action): web::Json<ActionRequest>) -> impl Responder {
    match Actions::new(action).insert_action().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
