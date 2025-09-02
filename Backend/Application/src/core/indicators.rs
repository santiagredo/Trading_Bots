use models::{
    entities::{
        indicators::{self, Model},
        pairs,
    },
    structs::{PairRequest, StrategyRequest, Ticker},
};

use crate::{
    config::get_config,
    handler::{Indicators, Pairs, Strategies, SubscribedIndicators},
    utils::{handle_user_err, Cache, Core, Data, Logic, Response},
};

impl Indicators<Core> {
    pub async fn insert_indicator_core(self) -> Result<Model, Response> {
        let Some(strategy) = Strategies::new(StrategyRequest {
            id: self.model.strategy_id,
            ..Default::default()
        })
        .select_strategy()
        .await?
        else {
            return Err(Response::not_found("Strategy".to_string()));
        };

        if Pairs::new(PairRequest {
            symbol: self.model.symbol.clone(),
            ..Default::default()
        })
        .select_pair()
        .await?
        .is_none()
        {
            return Err(Response::not_found("Pair".to_string()));
        };

        let indicator = self
            .next_phase::<Logic>()
            .insert_indicator_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_indicator_data(&get_config().await.db)
            .await?;

        match strategy.is_active {
            true => {
                let active_indicator = Indicators::<Cache>::set_active_indicator(indicator).await;
                let subscribed_indicator = SubscribedIndicators::default()
                    .update_subscribed_indicator(active_indicator)
                    .await;

                Ok(subscribed_indicator)
            }
            false => Ok(indicator),
        }
    }

    pub async fn select_indicator_core(self) -> Result<Option<Model>, Response> {
        let memory_indicator =
            Indicators::<Cache>::get_active_indicator(&self.model.strategy_id.unwrap_or_default())
                .await;

        if memory_indicator.is_some() {
            return Ok(memory_indicator);
        }

        self.next_phase::<Data>()
            .select_indicator_data(&get_config().await.db)
            .await
    }

    pub async fn select_indicators_core(self) -> Result<Vec<Model>, Response> {
        self.next_phase::<Data>()
            .select_indicators_data(&get_config().await.db)
            .await
    }

    pub async fn update_indicator_core(self) -> Result<Model, Response> {
        let Some(strategy) = Strategies::new(StrategyRequest {
            id: self.model.strategy_id,
            ..Default::default()
        })
        .select_strategy()
        .await?
        else {
            return Err(Response::not_found("Strategy".to_string()));
        };

        if Pairs::new(PairRequest {
            symbol: self.model.symbol.clone(),
            ..Default::default()
        })
        .select_pair()
        .await?
        .is_none()
        {
            return Err(Response::not_found("Pair".to_string()));
        };

        let mut indicator = self
            .next_phase::<Logic>()
            .update_indicator_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .insert_indicator_data(&get_config().await.db)
            .await?;

        if !strategy.is_active {
            indicator.is_active = false
        }

        let active_indicator = Indicators::<Cache>::set_active_indicator(indicator).await;
        let subscribed_indicator = SubscribedIndicators::default()
            .update_subscribed_indicator(active_indicator)
            .await;

        Ok(subscribed_indicator)
    }

    pub async fn delete_indicator_core(self) -> Result<u64, Response> {
        let mut indicator = Indicators::into_model(self.model.clone());
        indicator.is_active = false;

        let active_indicator = Indicators::<Cache>::set_active_indicator(indicator).await;
        SubscribedIndicators::default()
            .update_subscribed_indicator(active_indicator)
            .await;

        self.next_phase::<Logic>()
            .delete_indicator_logic()
            .map_err(handle_user_err)?
            .next_phase::<Data>()
            .delete_indicator_data(&get_config().await.db)
            .await
    }

    pub fn evaluate_indicator_core(
        self,
        indicator: &indicators::Model,
        ticker: &Ticker,
        pair: &pairs::Model,
    ) -> Result<bool, String> {
        self.next_phase::<Logic>()
            .evaluate_indicator_logic(indicator, ticker, pair)
    }
}
