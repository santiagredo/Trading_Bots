use std::{marker::PhantomData, sync::Arc};

use models::entities::assets::Model;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::utils::Core;

#[derive(Debug, Default)]
pub struct Assets<Phase = Core> {
    pub phase: PhantomData<Phase>,
    pub model: Model,
}

pub static ASSETS: Lazy<Arc<RwLock<Vec<Model>>>> = Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

impl Assets {
    pub async fn get_asset(asset_id: i32) -> Model {
        let asset = ASSETS.read().await;

        // if orders.is_empty() {
        //     let new_orders = Assets::<Core>::select_open_orders()
        //         .await
        //         .unwrap_or_default();

        //     *orders = new_orders;

        //     println!("Open orders: {orders:?} \n");
        // }

        asset
            .iter()
            .find(|asset| asset.id == asset_id)
            .cloned()
            .unwrap_or_default()
    }

    pub async fn reload_assets() {
        let mut assets = ASSETS.write().await;

        let stored_assets = Assets::<Core>::select_assets().await.unwrap_or_default();

        *assets = stored_assets;

        println!("Assets: {} \n", assets.iter().count());
    }
}
