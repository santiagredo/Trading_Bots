use actix_web::{get, web, HttpResponse, Responder};
use models::structs::{Environments, RecordTypeRequest};

use crate::{handler::RecordTypes, utils::error_response};

#[get("/{env}/all")]
pub async fn select_record_types(
    env: web::Path<Environments>,
    record_types: web::Query<RecordTypeRequest>,
) -> impl Responder {
    match RecordTypes::new(record_types.into_inner())
        .with_env(env.into_inner())
        .select_record_types()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
