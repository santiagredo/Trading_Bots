use std::{collections::HashMap, sync::Arc};

use models::entities::indicators::Model;
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{
    handler::{Indicators, Strategies},
    utils::{Cache, Response},
};

static ACTIVE_INDICATORS: Lazy<Arc<RwLock<Option<HashMap<i32, Model>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Indicators<Cache> {
    pub async fn set_active_indicators(indicators: Option<Vec<Model>>) -> Option<Vec<Model>> {
        let mut active_indicators = ACTIVE_INDICATORS.write().await;

        let Some(indicators) = indicators else {
            *active_indicators = None;
            return None;
        };

        let mut active_indicators_map: HashMap<i32, Model> = HashMap::new();

        for indicator in indicators.iter() {
            active_indicators_map.insert(indicator.strategy_id, indicator.clone());
        }

        // dbg!(&active_indicators_map);
        *active_indicators = Some(active_indicators_map);

        Some(indicators)
    }

    pub async fn set_active_indicator(indicator: Model) -> Model {
        let mut active_indicators = ACTIVE_INDICATORS.write().await;

        // early return if active indicators is none
        let Some(indicators_map) = active_indicators.as_mut() else {
            return indicator;
        };

        if !indicator.is_active {
            indicators_map.remove(&indicator.strategy_id);
        } else {
            indicators_map.insert(indicator.strategy_id, indicator.clone());
        }

        indicator
    }

    pub async fn get_active_indicators() -> Option<HashMap<i32, Model>> {
        let active_indicators = ACTIVE_INDICATORS.read().await;

        active_indicators.clone()
    }

    pub async fn get_active_indicator(key: &i32) -> Option<Model> {
        let active_indicators = ACTIVE_INDICATORS.read().await;

        let Some(indicators_map) = active_indicators.as_ref() else {
            return None;
        };

        indicators_map.get(key).cloned()
    }

    pub async fn start_active_indicators() -> Result<(), Response> {
        let active_strategies = Strategies::get_active_strategies_cache()
            .await
            .unwrap_or_default();

        if Self::get_active_indicators()
            .await
            .is_none_or(|active_indicators| active_indicators.is_empty())
        {
            let mut indicators_request = Indicators::default();
            indicators_request.model.is_active = Some(true);

            let active_indicators = indicators_request
                .select_indicators()
                .await?
                .into_iter()
                .filter(|act_ind| active_strategies.contains_key(&act_ind.strategy_id))
                .collect::<Vec<_>>();

            Self::set_active_indicators(Some(active_indicators)).await;
        }

        Ok(())
    }

    pub async fn stop_active_indicators() {
        Self::set_active_indicators(None).await;
    }
}
