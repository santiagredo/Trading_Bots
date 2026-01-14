use models::{entities::ledgers::Model, structs::QueryOptions};

use crate::{
    handler::{Ledgers, DBC},
    utils::{handle_user_err, Core, Response},
};

impl Ledgers<Core> {
    pub async fn insert_ledger_core(self) -> Result<Model, Response> {
        let env = self.environment;

        let logic_type = self
            .next_phase()
            .insert_ledger_logic()
            .map_err(handle_user_err)?;

        logic_type
            .next_phase()
            .insert_ledger_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_ledger_core(self) -> Result<Option<Model>, Response> {
        let env = self.environment;
        self.next_phase()
            .select_ledger_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_ledgers_core(
        self,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase()
            .select_ledgers_data(&DBC::db(&env).await?, query)
            .await
    }
}
