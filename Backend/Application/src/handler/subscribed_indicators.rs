use std::{
    collections::{HashMap, HashSet},
    marker::PhantomData,
};

use models::entities::indicators::{self, Model};

use crate::utils::Types;

#[derive(Debug)]
pub struct SubscribedIndicators<Phase = Types> {
    pub phase: PhantomData<Phase>,
}

impl<Phase> SubscribedIndicators<Phase> {
    pub fn next_phase<Next>(self) -> SubscribedIndicators<Next> {
        SubscribedIndicators {
            phase: PhantomData::<Next>,
        }
    }
}

impl SubscribedIndicators {
    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
        }
    }

    pub async fn update_subscribed_indicator(self, indicator: indicators::Model) -> Model {
        self.next_phase()
            .update_subscribed_indicator_core(indicator)
            .await
    }

    pub async fn select_subscribed_indicator(self, key: String) -> Option<HashSet<i32>> {
        self.next_phase()
            .select_subscribed_indicator_core(key)
            .await
    }

    pub async fn select_subscribed_indicators(self) -> Option<HashMap<String, HashSet<i32>>> {
        self.next_phase().select_subscribed_indicators_core().await
    }

    pub async fn start_subscribed_indicators(self) {
        self.next_phase().start_subscribed_indicators_core().await
    }

    pub async fn stop_subscribed_indicators(self) {
        self.next_phase().stop_subscribed_indicators_core().await
    }
}
