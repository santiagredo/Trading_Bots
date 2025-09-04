use actix_web::{get, web, HttpResponse, Responder};
use models::structs::TaskRequest;

use crate::{handler::Tasks, utils::error_response};

#[get("/all")]
pub async fn select_tasks(task: web::Query<TaskRequest>) -> impl Responder {
    match Tasks::new(task.into_inner()).select_tasks().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/memory/all")]
pub async fn select_active_tasks() -> impl Responder {
    let result = Tasks::default().select_active_tasks().await;
    HttpResponse::Ok().json(result)
}
