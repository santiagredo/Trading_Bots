use std::collections::{HashMap, HashSet};

use models::entities::indicators::{self, Model};

use crate::{
    handler::{Indicators, SubscribedIndicators},
    utils::{Cache, Core},
};

impl SubscribedIndicators<Core> {
    pub async fn set_active_subscribed_indicator_core(
        self,
        indicator: indicators::Model,
        is_remove: bool,
    ) -> Model {
        SubscribedIndicators::<Cache>::set_active_subscribed_indicator_cache(
            &self.environment,
            indicator,
            is_remove,
        )
        .await
    }

    pub async fn get_active_subscribed_indicator_core(self, key: String) -> Option<HashSet<i32>> {
        SubscribedIndicators::<Cache>::get_active_subscribed_indicator_cache(&self.environment, key)
            .await
    }

    pub async fn get_active_subscribed_indicators_core(self) -> Option<HashMap<String, HashSet<i32>>> {
        SubscribedIndicators::<Cache>::get_active_subscribed_indicators_cache(&self.environment)
            .await
    }

    pub async fn start_active_subscribed_indicators_core(self) {
        let env = self.environment;

        if SubscribedIndicators::<Cache>::get_active_subscribed_indicators_status_cache(&env).await
        {
            dbg!("Subscribed indicators already initialized");
            return;
        }

        let mut indicators_map: HashMap<String, HashSet<i32>> = HashMap::new();

        let mut indicators_request = Indicators::default();
        indicators_request.environment = env;

        let active_indicators = indicators_request
            .get_active_indicators()
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

        SubscribedIndicators::<Cache>::set_active_subscribed_indicators_cache(&env, indicators_map)
            .await;
    }

    pub async fn stop_active_subscribed_indicators_core(self) {
        SubscribedIndicators::<Cache>::stop_active_subscribed_indicators_cache(&self.environment)
            .await
    }
}
