use std::{marker::PhantomData, sync::Arc};

use models::entities::pair_assets::{self, Model};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::utils::Core;

#[derive(Debug, Default)]
pub struct PairAssets<Phase = Core> {
    pub phase: PhantomData<Phase>,
    pub model: Model,
}

pub static PAIR_ASSETS: Lazy<Arc<RwLock<Vec<Model>>>> =
    Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

impl PairAssets {
    pub async fn get_pair_asset(symbol: String) -> Model {
        let mut pair_assets = PAIR_ASSETS.write().await;

        if pair_assets.is_empty() {
            let stored_pair_assets = PairAssets::<Core>::select_all_pair_assets()
                .await
                .unwrap_or_default();

            *pair_assets = stored_pair_assets;

            println!("Pair assets: {pair_assets:?}");
        }

        pair_assets
            .iter()
            .find(|pair| pair.symbol == symbol)
            .map_or_else(pair_assets::Model::default, |pair| pair.to_owned())
    }

    pub async fn reload_pair_assets() {
        let mut pair_assets = PAIR_ASSETS.write().await;

        let stored_pair_assets = PairAssets::<Core>::select_all_pair_assets()
            .await
            .unwrap_or_default();

        *pair_assets = stored_pair_assets;

        println!("Pair assets: {pair_assets:?}");
    }
}
