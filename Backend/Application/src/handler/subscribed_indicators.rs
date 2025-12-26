use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use models::{
    entities::indicators::{self, Model},
    structs::Environments,
};

use crate::utils::Types;

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

    pub async fn set_active_subscribed_indicator(
        self,
        indicator: indicators::Model,
        is_remove: bool,
    ) -> Model {
        self.next_phase()
            .set_active_subscribed_indicator_core(indicator, is_remove)
            .await
    }

    pub async fn get_active_subscribed_indicator(self, key: String) -> Option<HashSet<i32>> {
        self.next_phase()
            .get_active_subscribed_indicator_core(key)
            .await
    }

    pub async fn get_active_subscribed_indicators(self) -> Option<HashMap<String, HashSet<i32>>> {
        self.next_phase()
            .get_active_subscribed_indicators_core()
            .await
    }

    pub async fn start_active_subscribed_indicators(self) {
        self.next_phase()
            .start_active_subscribed_indicators_core()
            .await
    }

    pub async fn stop_active_subscribed_indicators(self) {
        self.next_phase()
            .stop_active_subscribed_indicators_core()
            .await
    }
}
