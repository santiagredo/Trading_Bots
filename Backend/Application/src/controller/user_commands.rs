use actix_web::{post, web, HttpResponse, Responder};
use models::structs::Environments;

use crate::handler::UserCommands;

#[post("/{env}/start")]
pub async fn start_everything(env: web::Path<Environments>) -> impl Responder {
    match UserCommands::start_everything(false, env.into_inner()).await {
        Err(err) => HttpResponse::InternalServerError().json(err),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}

#[post("/{env}/stop")]
pub async fn stop_everything(env: web::Path<Environments>) -> impl Responder {
    match UserCommands::stop_everything(env.into_inner()).await {
        Err(err) => HttpResponse::InternalServerError().json(err),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}

#[post("/{env}/restart")]
pub async fn restart_everything(env: web::Path<Environments>) -> impl Responder {
    match UserCommands::restart_everything(false, env.into_inner()).await {
        Err(err) => HttpResponse::InternalServerError().json(err),
        Ok(_) => HttpResponse::Ok().finish(),
    }
}
