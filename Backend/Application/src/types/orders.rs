use std::{marker::PhantomData, sync::Arc};

use models::entities::orders::Model;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::utils::Core;

#[derive(Debug, Default)]
pub struct Orders<Phase = Core> {
    pub phase: PhantomData<Phase>,
    pub model: Model,
}

pub static ORDERS: Lazy<Arc<RwLock<Vec<Model>>>> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

impl Orders {
    pub async fn get_stored_orders(base_asset_id: i32, quote_asset_id: i32) -> Vec<Model> {
        let orders = ORDERS.read().await;

        orders
            .iter()
            .filter(|order| {
                order.base_asset_id == base_asset_id && order.quote_asset_id == quote_asset_id
            })
            .cloned()
            .collect()
    }

    pub async fn reload_stored_orders() {
        let mut all_orders = Vec::new();

        let mut open_orders = Orders::<Core>::select_open_orders()
            .await
            .unwrap_or_default();

        all_orders.append(&mut open_orders);

        let mut last_order_by_strategy = Orders::<Core>::select_completed_orders()
            .await
            .unwrap_or_default();

        all_orders.append(&mut last_order_by_strategy);

        let mut orders = ORDERS.write().await;

        *orders = all_orders;

        println!("Recent orders: {} \n", orders.iter().count());
    }
}
