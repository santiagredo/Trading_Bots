use actix_web::{delete, get, post, put, web, HttpResponse, Responder};
use models::structs::IndicatorRequest;

use crate::{
    handler::{Indicators, SubscribedIndicators},
    utils::error_response,
};

#[post("")]
pub async fn insert_indicator(web::Json(indicator): web::Json<IndicatorRequest>) -> impl Responder {
    match Indicators::new(indicator).insert_indicator().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("")]
pub async fn select_indicator(query: web::Query<IndicatorRequest>) -> impl Responder {
    match Indicators::new(query.into_inner()).select_indicator().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[get("/all")]
pub async fn select_indicators(query: web::Query<IndicatorRequest>) -> impl Responder {
    match Indicators::new(query.into_inner())
        .select_indicators()
        .await
    {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[put("")]
pub async fn update_indicator(web::Json(indicator): web::Json<IndicatorRequest>) -> impl Responder {
    match Indicators::new(indicator).update_indicator().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

#[delete("")]
pub async fn delete_indicator(web::Json(indicator): web::Json<IndicatorRequest>) -> impl Responder {
    match Indicators::new(indicator).delete_indicator().await {
        Ok(val) => HttpResponse::Ok().json(val),
        Err(err) => error_response(err),
    }
}

// #[get("/all")]
// pub async fn select_all_indicators() -> impl Responder {
//     match Indicators::new(IndicatorRequest::default())
//         .select_all_indicators()
//         .await
//     {
//         Ok(val) => HttpResponse::Ok().json(val),
//         Err(err) => error_response(err),
//     }
// }

#[get("/memory")]
pub async fn get_subscribed_indicators() -> impl Responder {
    let val = SubscribedIndicators::default()
        .select_subscribed_indicators()
        .await;
    HttpResponse::Ok().json(val)
}
