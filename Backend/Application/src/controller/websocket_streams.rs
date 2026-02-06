use actix_web::{get, HttpResponse, Responder};

use crate::handler::WebsocketStreams;

// cache
#[get("/memory")]
pub async fn get_websocket_status() -> impl Responder {
    let status = WebsocketStreams::new().get_status().await;
    HttpResponse::Ok().json(status)
}
