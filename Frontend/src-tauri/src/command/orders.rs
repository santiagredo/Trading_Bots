use crate::{
    core::{insert_order_core, select_order_core, select_orders_core, update_order_core},
    models::{entities::orders::Model, structs::OrderRequest},
};

#[tauri::command]
pub async fn insert_order(env: String, order: OrderRequest) -> Result<Model, String> {
    insert_order_core(env, order).await
}

#[tauri::command]
pub async fn select_order(env: String, query: OrderRequest) -> Result<Model, String> {
    select_order_core(env, query).await
}

#[tauri::command]
pub async fn select_orders(env: String, query: Option<OrderRequest>) -> Result<Vec<Model>, String> {
    select_orders_core(env, query).await
}

#[tauri::command]
pub async fn update_order(env: String, order: OrderRequest) -> Result<Model, String> {
    update_order_core(env, order).await
}
