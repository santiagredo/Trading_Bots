use actix_web::{get, web, HttpResponse, Responder};
use models::structs::Environments;

use crate::{handler::ErrorLogs, utils::error_response};

#[get("/{env}/all")]
pub async fn select_error_logs(env: web::Path<Environments>) -> impl Responder {
    match ErrorLogs::default()
        .with_env(env.into_inner())
        .select_logs()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
