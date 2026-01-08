use crate::{
    integration::{
        insert_order_integration, select_order_integration, select_orders_integration,
        update_order_integration,
    },
    models::{entities::orders::Model, structs::OrderRequest},
    utils::handle_response,
};

// db
pub async fn insert_order_core(env: String, order: OrderRequest) -> Result<Model, String> {
    let response = insert_order_integration(env, order).await?;
    handle_response::<Model>(response).await
}

pub async fn select_order_core(env: String, query: OrderRequest) -> Result<Model, String> {
    let response = select_order_integration(env, query).await?;
    handle_response::<Model>(response).await
}

pub async fn select_orders_core(env: String, query: Option<OrderRequest>) -> Result<Vec<Model>, String> {
    let response = select_orders_integration(env, query).await?;
    handle_response::<Vec<Model>>(response).await
}

pub async fn update_order_core(env: String, order: OrderRequest) -> Result<Model, String> {
    let response = update_order_integration(env, order).await?;
    handle_response::<Model>(response).await
}
