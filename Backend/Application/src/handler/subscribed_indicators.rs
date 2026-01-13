use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use models::{entities::indicators::Model, enums::LifecycleState, structs::Environments};

use crate::utils::{Core, Response, Types};

#[derive(Debug)]
pub struct SubscribedIndicators<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
}

impl<Phase> SubscribedIndicators<Phase> {
    pub fn next_phase<Next>(self) -> SubscribedIndicators<Next> {
        SubscribedIndicators {
            phase: PhantomData::<Next>,
            environment: self.environment,
        }
    }
}

impl SubscribedIndicators {
    pub fn new(environment: Environments) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment,
        }
    }

    /* ===========================
     * CACHE (WRITE)
     * ===========================
     */

    pub async fn upsert_subscribed_indicator(
        self,
        indicator: Model,
    ) -> Result<(), crate::utils::Response> {
        self.next_phase()
            .upsert_subscribed_indicator_core(indicator)
            .await
    }

    pub async fn remove_subscribed_indicator(
        self,
        indicator: Model,
    ) -> Result<Option<Model>, crate::utils::Response> {
        self.next_phase()
            .remove_subscribed_indicator_core(indicator)
            .await
    }

    /* ===========================
     * CACHE (READ)
     * ===========================
     */

    pub async fn get_subscribed_indicator(self, key: String) -> Option<HashSet<i32>> {
        self.next_phase().get_subscribed_indicator_core(key).await
    }

    pub async fn get_subscribed_indicators(self) -> Option<HashMap<String, HashSet<i32>>> {
        self.next_phase().get_subscribed_indicators_core().await
    }

    pub async fn get_subscribed_indicators_state(self) -> LifecycleState {
        self.next_phase()
            .get_subscribed_indicators_state_core()
            .await
    }

    /* ===========================
     * LIFECYCLE
     * ===========================
     */

    pub async fn start_subscribed_indicators(self) -> Result<(), Response> {
        self.next_phase().start_subscribed_indicators_core().await
    }

    pub async fn stop_subscribed_indicators(self) -> Result<(), Response> {
        self.next_phase().stop_subscribed_indicators_core().await
    }

    pub async fn reset_subscribed_indicators(self) -> Result<(), Response> {
        self.next_phase().reset_subscribed_indicators_core().await
    }

    pub async fn get_all_unique_symbols() -> Result<Vec<String>, String> {
        SubscribedIndicators::<Core>::get_all_unique_symbols_core().await
    }
}
