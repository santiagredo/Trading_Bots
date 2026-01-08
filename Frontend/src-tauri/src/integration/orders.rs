use reqwest::{Client, Response};

use crate::{
    models::structs::OrderRequest,
    static_strings::{BACKEND_URL, ORDERS},
};

pub async fn insert_order_integration(
    env: String,
    order: OrderRequest,
) -> Result<Response, String> {
    Client::new()
        .post(format!("{BACKEND_URL}{ORDERS}/{env}"))
        .json(&order)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_order_integration(
    env: String,
    query: OrderRequest,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{ORDERS}/{env}"))
        .query(&query)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn select_orders_integration(
    env: String,
    query: Option<OrderRequest>,
) -> Result<Response, String> {
    Client::new()
        .get(format!("{BACKEND_URL}{ORDERS}/{env}/all"))
        .query(&query)
        .send()
        .await
        .map_err(|err| err.to_string())
}

pub async fn update_order_integration(
    env: String,
    order: OrderRequest,
) -> Result<Response, String> {
    Client::new()
        .patch(format!("{BACKEND_URL}{ORDERS}/{env}"))
        .json(&order)
        .send()
        .await
        .map_err(|err| err.to_string())
}
