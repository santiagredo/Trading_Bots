use models::entities::ledgers::Model;

use crate::{
    config::get_config,
    handler::Ledgers,
    utils::{handle_user_err, Core, Response},
};

impl Ledgers<Core> {
    pub async fn insert_ledger_core(self) -> Result<Model, Response> {
        let logic_type = self
            .next_phase()
            .insert_ledger_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase()
            .insert_ledger_data(&get_config().await.db)
            .await
    }

    pub async fn select_ledger_core(self) -> Result<Option<Model>, Response> {
        self.next_phase()
            .select_ledger_data(&get_config().await.db)
            .await
    }

    pub async fn select_ledgers_core(self) -> Result<Vec<Model>, Response> {
        self.next_phase()
            .select_ledgers_data(&get_config().await.db)
            .await
    }
}
