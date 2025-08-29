use models::{
    entities::{
        indicators::{self, Model},
        pairs,
    },
    structs::{PairRequest, StrategyRequest, Ticker},
};

use crate::{
    config::get_config,
    types::{Indicators, Pairs, Strategies},
    utils::{handle_user_err, Core, Data, Logic, Response},
};

impl Indicators<Core> {
    pub async fn insert_indicator_core(self) -> Result<Model, Response> {
        Strategies::new(StrategyRequest {
            id: self.model.strategy_id,
            ..Default::default()
        })
        .select_strategy()
        .await?;

        Pairs::<Core> {
            model: PairRequest {
                symbol: self.model.symbol.clone(),
                ..Default::default()
            },
            phase: std::marker::PhantomData,
        }
        .select_pair_core()
        .await?;

        let logic_type = self
            .next_phase::<Logic>()
            .insert_indicator_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .insert_indicator_data(&get_config().await.db)
            .await
    }

    pub async fn select_indicator_core(self) -> Result<Option<Model>, Response> {
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
        Strategies::new(StrategyRequest {
            id: self.model.strategy_id,
            ..Default::default()
        })
        .select_strategy()
        .await?;

        Pairs::<Core> {
            model: PairRequest {
                symbol: self.model.symbol.clone(),
                ..Default::default()
            },
            phase: std::marker::PhantomData,
        }
        .select_pair_core()
        .await?;

        let logic_type = self
            .next_phase::<Logic>()
            .update_indicator_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase::<Data>()
            .insert_indicator_data(&get_config().await.db)
            .await
    }

    pub async fn delete_indicator_core(self) -> Result<u64, Response> {
        let logic_type = self
            .next_phase::<Logic>()
            .delete_indicator_logic()
            .map_err(handle_user_err)?;

        logic_type
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
