use models::entities::ledgers::Model;

use crate::{
    config::get_config,
    types::Ledgers,
    utils::{Core, Data, Logic, Outcome, OutcomeError},
};

impl Ledgers<Core> {
    pub async fn insert_ledger(ledger: Model) -> Outcome<Model, String, String> {
        let data_type =
            Ledgers::<Logic>::insert_ledger(ledger).map_err(|err| OutcomeError::Failure(err))?;

        Ledgers::<Data>::insert_ledger(&get_config().await.db, data_type).await
    }

    pub async fn select_ledger(id: i32) -> Outcome<Model, String, String> {
        Ledgers::<Logic>::select_ledger(id).map_err(|err| OutcomeError::Failure(err))?;

        Ledgers::<Data>::select_ledger(&get_config().await.db, id).await
    }

    pub async fn select_ledgers() -> Outcome<Vec<Model>, String, String> {
        // Ledgers::<Logic>::select_ledgers().map_err(|err| OutcomeError::Failure(err))?;

        Ledgers::<Data>::select_ledgers(&get_config().await.db).await
    }
}
