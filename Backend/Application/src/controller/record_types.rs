use actix_web::{get, web, HttpResponse, Responder};
use models::structs::Environments;

use crate::{handler::RecordTypes, utils::error_response};

#[get("/{env}/all")]
pub async fn select_record_types(env: web::Path<Environments>) -> impl Responder {
    match RecordTypes::select(env.into_inner()).await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
