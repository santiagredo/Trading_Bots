use std::{collections::HashMap, sync::Arc};

use models::entities::pairs::Model;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{
    handler::Pairs,
    utils::{Cache, Response},
};

static ACTIVE_PAIRS: Lazy<Arc<RwLock<Option<HashMap<i32, Model>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Pairs<Cache> {
    async fn set_active_pairs(pairs: Option<Vec<Model>>) -> Option<Vec<Model>> {
        let mut memory_pairs = ACTIVE_PAIRS.write().await;

        let Some(pairs) = pairs else {
            *memory_pairs = None;
            return None;
        };

        let mut active_pairs_map: HashMap<i32, Model> = HashMap::new();

        for pair in pairs.iter() {
            active_pairs_map.insert(pair.id, pair.clone());
        }

        *memory_pairs = Some(active_pairs_map);
        Some(pairs)
    }

    pub async fn set_active_pair(pair: Model, is_remove: bool) -> Model {
        let mut memory_pairs = ACTIVE_PAIRS.write().await;

        let Some(pairs_map) = memory_pairs.as_mut() else {
            return pair;
        };

        if is_remove {
            pairs_map.remove(&pair.id);
        } else {
            pairs_map.insert(pair.id, pair.clone());
        }

        pair
    }

    pub async fn get_active_pairs() -> Option<HashMap<i32, Model>> {
        let memory_pairs = ACTIVE_PAIRS.read().await;

        memory_pairs.clone()
    }

    pub async fn get_active_pair(key: &i32) -> Option<Model> {
        let active_pairs = ACTIVE_PAIRS.read().await;

        let Some(pairs_map) = active_pairs.as_ref() else {
            return None;
        };

        pairs_map.get(key).cloned()
    }

    pub async fn start_active_pairs() -> Result<(), Response> {
        if Self::get_active_pairs()
            .await
            .is_none_or(|pairs| pairs.is_empty())
        {
            let pairs = Pairs::default().select_pairs().await?;
            Self::set_active_pairs(Some(pairs)).await;
        }

        Ok(())
    }

    pub async fn stop_active_pairs() {
        Self::set_active_pairs(None).await;
    }
}
