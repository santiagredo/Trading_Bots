use actix_web::{get, web, HttpResponse, Responder};
use models::structs::{Environments, QueryOptions};

use crate::{handler::IntegrationLogs, utils::error_response};

#[get("/{env}/all")]
pub async fn select_integration_logs(
    env: web::Path<Environments>,
    query: web::Query<QueryOptions>,
) -> impl Responder {
    match IntegrationLogs::default()
        .with_env(env.into_inner())
        .select_logs(Some(query.into_inner()))
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}
