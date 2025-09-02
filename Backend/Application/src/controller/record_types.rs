use actix_web::{get, web, HttpResponse, Responder};
use models::structs::RecordTypeRequest;

use crate::{handler::RecordTypes, utils::error_response};

#[get("/all")]
pub async fn select_record_types(record_types: web::Query<RecordTypeRequest>) -> impl Responder {
    match RecordTypes::new(record_types.into_inner())
        .select_record_types()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
