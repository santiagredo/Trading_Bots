use crate::{
    handler::Binance,
    utils::{error_response, RepoFactory},
};
use actix_web::{get, web, HttpResponse, Responder};
use models::structs::Environments;

#[get("/{env}")]
pub async fn get_account(env: web::Path<Environments>) -> impl Responder {
    let env = env.into_inner();

    let factory = match RepoFactory::db(env).await {
        Err(err) => return error_response(err),
        Ok(val) => val,
    };

    match Binance::blank().resolve_account(factory, env).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
