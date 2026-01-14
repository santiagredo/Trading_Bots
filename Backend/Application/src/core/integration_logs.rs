use models::{entities::integration_log::Model, structs::QueryOptions};

use crate::{
    handler::{IntegrationLogs, DBC},
    utils::{Core, Response},
};

impl IntegrationLogs<Core> {
    pub async fn insert_log_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase()
            .insert_log_data(&DBC::db(&env).await?)
            .await
    }

    pub async fn select_logs_core(
        self,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase()
            .select_logs_data(&DBC::db(&env).await?, query)
            .await
    }
}
