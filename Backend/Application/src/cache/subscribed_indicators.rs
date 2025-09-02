use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    sync::Arc,
};

use models::entities::indicators::Model;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{
    handler::{Indicators, SubscribedIndicators},
    utils::Cache,
};

static SUBSCRIBED_INDICATORS: Lazy<Arc<RwLock<Option<HashMap<String, HashSet<i32>>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl SubscribedIndicators<Cache> {
    async fn set_subscribed_indicators_cache(
        indicators_map: Option<HashMap<String, HashSet<i32>>>,
    ) -> Option<HashMap<String, HashSet<i32>>> {
        let mut indicators = SUBSCRIBED_INDICATORS.write().await;
        *indicators = indicators_map;
        indicators.clone()
    }

    pub async fn set_subscribed_indicator_cache(indicator: Model) -> Model {
        let mut subscribed_indicators = SUBSCRIBED_INDICATORS.write().await;
        let Some(map) = subscribed_indicators.as_mut() else {
            return indicator;
        };

        match map.entry(indicator.symbol.clone()) {
            Entry::Occupied(mut entry) if !indicator.is_active => {
                entry.get_mut().remove(&indicator.strategy_id);

                if entry.get().is_empty() {
                    entry.remove_entry();
                }
            }
            Entry::Vacant(entry) if indicator.is_active => {
                entry.insert([indicator.strategy_id].into_iter().collect());
            }
            Entry::Occupied(mut entry) if indicator.is_active => {
                entry.get_mut().insert(indicator.strategy_id);
            }
            _ => {}
        }

        indicator
    }

    pub async fn get_subscribed_indicators_cache() -> Option<HashMap<String, HashSet<i32>>> {
        let indicators = SUBSCRIBED_INDICATORS.read().await;
        indicators.clone()
    }

    pub async fn get_subscribed_indicator_cache(key: String) -> Option<HashSet<i32>> {
        let indicators = SUBSCRIBED_INDICATORS.read().await;

        let Some(indicators_map) = indicators.as_ref() else {
            return None;
        };

        indicators_map.get(&key).cloned()
    }

    pub async fn start_subscribed_indicators_cache() {
        if SubscribedIndicators::<Cache>::get_subscribed_indicators_cache()
            .await
            .is_none_or(|sub_indicators| sub_indicators.is_empty())
        {
            let mut indicators_map: HashMap<String, HashSet<i32>> = HashMap::new();

            let active_indicators = Indicators::<Cache>::get_active_indicators()
                .await
                .unwrap_or_default();

            for (_, indicator) in active_indicators {
                indicators_map
                    .entry(indicator.symbol)
                    .and_modify(|val| {
                        val.insert(indicator.strategy_id);
                    })
                    .or_insert_with(HashSet::new)
                    .insert(indicator.strategy_id);
            }

            SubscribedIndicators::<Cache>::set_subscribed_indicators_cache(Some(indicators_map))
                .await;
        }
    }

    pub async fn stop_subscribed_indicators_cache() {
        SubscribedIndicators::<Cache>::set_subscribed_indicators_cache(None).await;
    }
}
