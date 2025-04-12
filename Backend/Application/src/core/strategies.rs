use models::{
    entities::{
        self, assets, orders, pair_assets,
        strategies::{self, Model},
    },
    structs::Ticker,
};
use sea_orm::prelude::Decimal;

use crate::{
    config::get_config,
    types::Strategies,
    utils::{Core, Data, Logic, Outcome, OutcomeError},
};

impl Strategies<Core> {
    pub async fn insert_strategy(strategy: Model) -> Outcome<Model, String, String> {
        let data_type = Strategies::<Logic>::insert_strategy(strategy)
            .map_err(|err| OutcomeError::Failure(err))?;

        Strategies::<Data>::insert_strategy(&get_config().await.db, data_type).await
    }

    pub async fn select_strategy(id: i32) -> Outcome<Model, String, String> {
        Strategies::<Logic>::select_strategy(id).map_err(|err| OutcomeError::Failure(err))?;

        Strategies::<Data>::select_strategy(&get_config().await.db, id).await
    }

    pub async fn select_active_strategies() -> Outcome<Vec<Model>, String, String> {
        // Strategies::<Logic>::select_active_strategies().map_err(|err| OutcomeError::Failure(""))?;

        Strategies::<Data>::select_active_strategies(&get_config().await.db).await
    }

    pub async fn update_strategy(strategy: Model) -> Outcome<Model, String, String> {
        let data_type = Strategies::<Logic>::update_strategy(strategy)
            .map_err(|err| OutcomeError::Failure(err))?;

        Strategies::<Data>::update_strategy(&get_config().await.db, data_type).await
    }

    pub async fn delete_strategy(strategy: Model) -> Outcome<u64, String, String> {
        let data_type = Strategies::<Logic>::delete_strategy(strategy)
            .map_err(|err| OutcomeError::Failure(err))?;

        Strategies::<Data>::delete_strategy(&get_config().await.db, data_type).await
    }

    pub fn check_price_action_strategies(
        strategy: &strategies::Model,
        ticker: &Ticker,
        open_orders: &Vec<orders::Model>,
        pair_asset: &pair_assets::Model,
        quote_asset: &assets::Model,
    ) -> Vec<orders::Model> {
        Strategies::<Logic>::check_price_action_strategies(
            strategy,
            ticker,
            open_orders,
            pair_asset,
            quote_asset,
        )
    }

    pub fn check_open_orders(
        open_orders: Vec<orders::Model>,
        last_price: Decimal,
    ) -> Vec<entities::orders::Model> {
        Strategies::<Logic>::check_open_orders(open_orders, last_price)
    }

    // pub fn check_completed_orders_strategies(
    //     orders: Vec<entities::orders::Model>,
    //     c: Decimal,
    //     o: Decimal,
    //     h: Decimal,
    //     l: Decimal,
    // ) -> Vec<entities::orders::Model> {
    //     Strategies::<Logic>::check_completed_orders_strategies(orders, c, o, h, l)
    // }
}
