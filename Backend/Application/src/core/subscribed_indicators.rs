use std::collections::{HashMap, HashSet};

use models::entities::indicators::{self, Model};

use crate::{
    handler::SubscribedIndicators,
    utils::{Cache, Core},
};

impl SubscribedIndicators<Core> {
    pub async fn update_subscribed_indicator_core(self, indicator: indicators::Model) -> Model {
        SubscribedIndicators::<Cache>::set_subscribed_indicator_cache(indicator).await
    }

    pub async fn select_subscribed_indicator_core(self, key: String) -> Option<HashSet<i32>> {
        SubscribedIndicators::<Cache>::get_subscribed_indicator_cache(key).await
    }

    pub async fn select_subscribed_indicators_core(self) -> Option<HashMap<String, HashSet<i32>>> {
        SubscribedIndicators::<Cache>::get_subscribed_indicators_cache().await
    }

    pub async fn start_subscribed_indicators_core(self) {
        SubscribedIndicators::<Cache>::start_subscribed_indicators_cache().await
    }

    pub async fn stop_subscribed_indicators_core(self) {
        SubscribedIndicators::<Cache>::stop_subscribed_indicators_cache().await
    }
}
