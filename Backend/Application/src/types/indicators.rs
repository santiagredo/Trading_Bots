use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    marker::PhantomData,
    sync::Arc,
};

use models::{
    entities::{
        indicators::{self, Model},
        pairs,
    },
    structs::{IndicatorRequest, Ticker},
};
use once_cell::sync::Lazy;
use tokio::sync::RwLock;

use crate::{
    types::Strategies,
    utils::{Response, Types},
};

#[derive(Debug, Default)]
pub struct Indicators<Phase = Types> {
    pub phase: PhantomData<Phase>,
    pub model: IndicatorRequest,
}

static SUBSCRIBED_INDICATORS: Lazy<Arc<RwLock<Option<HashMap<String, HashSet<i32>>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

static ACTIVE_INDICATORS: Lazy<Arc<RwLock<Option<HashMap<i32, Model>>>>> =
    Lazy::new(|| Arc::new(RwLock::new(None)));

impl Indicators {
    pub fn new(model: IndicatorRequest) -> Self {
        Self {
            phase: PhantomData::<Types>,
            model,
        }
    }

    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            model: IndicatorRequest {
                ..Default::default()
            },
        }
    }

    pub fn into_model(indicator: IndicatorRequest) -> Model {
        Model {
            id: indicator.id.unwrap_or_default(),
            strategy_id: indicator.strategy_id.unwrap_or_default(),
            is_active: indicator.is_active.unwrap_or_default(),
            symbol: indicator.symbol.unwrap_or_default(),
            nick: indicator.nick.unwrap_or_default(),
            direction: indicator.direction.unwrap_or_default(),
            is_percentage: indicator.is_percentage.unwrap_or_default(),
            value: indicator.value.unwrap_or_default(),
        }
    }

    pub async fn set_subscribed_indicators(
        indicators_map: Option<HashMap<String, HashSet<i32>>>,
    ) -> Option<HashMap<String, HashSet<i32>>> {
        let mut indicators = SUBSCRIBED_INDICATORS.write().await;
        *indicators = indicators_map;
        indicators.clone()
    }

    pub async fn set_subscribed_indicator(indicator: Model) -> Model {
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

    pub async fn get_subscribed_indicators() -> Option<HashMap<String, HashSet<i32>>> {
        let indicators = SUBSCRIBED_INDICATORS.read().await;
        indicators.clone()
    }

    pub async fn get_subscribed_indicator(key: String) -> Option<HashSet<i32>> {
        let indicators = SUBSCRIBED_INDICATORS.read().await;

        let Some(indicators_map) = indicators.as_ref() else {
            return None;
        };

        indicators_map.get(&key).cloned()
    }

    pub async fn start_subscribed_indicators() {
        if Self::get_subscribed_indicators()
            .await
            .is_none_or(|sub_indicators| sub_indicators.is_empty())
        {
            let mut indicators_map: HashMap<String, HashSet<i32>> = HashMap::new();

            let active_indicators = Self::get_active_indicators().await.unwrap_or_default();

            for (_, indicator) in active_indicators {
                indicators_map
                    .entry(indicator.symbol)
                    .and_modify(|val| {
                        val.insert(indicator.strategy_id);
                    })
                    .or_insert_with(HashSet::new)
                    .insert(indicator.strategy_id);
            }

            Self::set_subscribed_indicators(Some(indicators_map)).await;
        }
    }

    pub async fn stop_subscribed_indicators() {
        Self::set_subscribed_indicators(None).await;
    }

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
        let active_strategies = Strategies::get_active_strategies()
            .await
            .unwrap_or_default();

        if Self::get_active_indicators()
            .await
            .is_none_or(|active_indicators| active_indicators.is_empty())
        {
            let mut indicators_request = Self::default();
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

    pub fn evalute_active_indicators(
        ticker: &Ticker,
        indicator: &indicators::Model,
        pair: &pairs::Model,
    ) -> Result<bool, String> {
        Self::default().evaluate_indicator(&indicator, &ticker, &pair)
    }
}

impl<Phase> Indicators<Phase> {
    pub fn next_phase<Next>(self) -> Indicators<Next> {
        Indicators {
            phase: PhantomData::<Next>,
            model: self.model,
        }
    }
}

impl Indicators<Types> {
    pub async fn insert_indicator(self) -> Result<Model, Response> {
        let indicator = self.next_phase().insert_indicator_core().await?;

        if Strategies::get_active_strategy(&indicator.strategy_id)
            .await
            .is_some_and(|strategy| strategy.is_active)
        {
            let active_indicator = Self::set_active_indicator(indicator).await;
            let subscribed_indicator = Self::set_subscribed_indicator(active_indicator).await;

            return Ok(subscribed_indicator);
        }

        Ok(indicator)
    }

    pub async fn select_indicator(self) -> Result<Option<Model>, Response> {
        let memory_indicator = Self::get_active_indicator(&self.model.id.unwrap_or_default()).await;

        if memory_indicator.is_some() {
            return Ok(memory_indicator);
        }

        self.next_phase().select_indicator_core().await
    }

    pub async fn select_indicators(self) -> Result<Vec<Model>, Response> {
        self.next_phase().select_indicators_core().await
    }

    pub async fn update_indicator(self) -> Result<Model, Response> {
        let mut indicator = self.next_phase().update_indicator_core().await?;

        if Strategies::get_active_strategy(&indicator.strategy_id)
            .await
            .is_none_or(|strategy| !strategy.is_active)
        {
            indicator.is_active = false;
        }

        let active_indicator = Self::set_active_indicator(indicator).await;
        let subscribed_indicator = Self::set_subscribed_indicator(active_indicator).await;

        Ok(subscribed_indicator)
    }

    pub async fn delete_indicator(self) -> Result<u64, Response> {
        let mut indicator = Self::into_model(self.model.clone());
        indicator.is_active = false;

        let active_indicator = Self::set_active_indicator(indicator).await;
        Self::set_subscribed_indicator(active_indicator).await;

        self.next_phase().delete_indicator_core().await
    }

    pub fn evaluate_indicator(
        self,
        indicator: &indicators::Model,
        ticker: &Ticker,
        pair: &pairs::Model,
    ) -> Result<bool, String> {
        self.next_phase()
            .evaluate_indicator_core(indicator, ticker, pair)
    }
}
