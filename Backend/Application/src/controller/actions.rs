use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use models::structs::{ActionRequest, Environments, QueryOptions};

use crate::{
    handler::Actions,
    utils::{error_response, DbRepo, EntityCache},
};

// ============================================
// DATABASE OPERATIONS
// ============================================

#[post("/{env}")]
pub async fn insert_action(
    env: web::Path<Environments>,
    web::Json(action): web::Json<ActionRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Actions::new(repo);

    match service.insert(action).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}")]
pub async fn select_action(
    env: web::Path<Environments>,
    action: web::Query<ActionRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Actions::new(repo);

    match service.select(action.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/{env}/all")]
pub async fn select_actions(
    env: web::Path<Environments>,
    action: web::Query<ActionRequest>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Actions::new(repo);

    match service
        .select_many(action.into_inner(), Some(query.into_inner()))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[patch("/{env}")]
pub async fn update_action(
    env: web::Path<Environments>,
    web::Json(action): web::Json<ActionRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Actions::new(repo);

    match service.update(action).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("/{env}")]
pub async fn delete_action(
    env: web::Path<Environments>,
    web::Json(action): web::Json<ActionRequest>,
) -> impl Responder {
    let repo = match DbRepo::new(env.into_inner()).await {
        Ok(val) => val,
        Err(err) => return error_response(err),
    };

    let service = Actions::new(repo);

    match service.delete(action).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// ============================================
// CACHE OPERATIONS
// ============================================

#[get("/{env}/memory")]
pub async fn get_action(
    env: web::Path<Environments>,
    action: web::Query<ActionRequest>,
) -> impl Responder {
    let action_id = match action.id {
        Some(id) => id,
        None => return HttpResponse::BadRequest().body("Action ID is required"),
    };

    match Actions::blank().get(env.into_inner(), action_id).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/{env}/memory/all")]
pub async fn get_actions(env: web::Path<Environments>) -> impl Responder {
    match Actions::blank().get_all(env.into_inner()).await {
        Some(val) => HttpResponse::Ok().json(val),
        None => HttpResponse::NotFound().finish(),
    }
}
