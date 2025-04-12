use models::entities::orders::Model;

use crate::{
    config::get_config,
    types::Orders,
    utils::{Core, Data, Logic, Outcome, OutcomeError},
};

impl Orders<Core> {
    pub async fn insert_order(order: Model) -> Outcome<Model, String, String> {
        let data_type =
            Orders::<Logic>::insert_order(order).map_err(|err| OutcomeError::Failure(err))?;

        Orders::<Data>::insert_order(&get_config().await.db, data_type).await
    }

    pub async fn select_order(id: i32) -> Outcome<Model, String, String> {
        Orders::<Logic>::select_order(id).map_err(|err| OutcomeError::Failure(err))?;

        Orders::<Data>::select_order(&get_config().await.db, id).await
    }

    pub async fn select_open_orders() -> Outcome<Vec<Model>, String, String> {
        Orders::<Data>::select_open_orders(&get_config().await.db).await
    }

    pub async fn select_completed_orders() -> Outcome<Vec<Model>, String, String> {
        Orders::<Data>::select_completed_orders(&get_config().await.db).await
    }

    pub async fn update_order(order: Model) -> Outcome<Model, String, String> {
        let data_type =
            Orders::<Logic>::update_order(order).map_err(|err| OutcomeError::Failure(err))?;

        Orders::<Data>::update_order(&get_config().await.db, data_type).await
    }
}
