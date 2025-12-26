use models::entities::error_log::Model;

use crate::{
    handler::{ErrorLogs, DBC},
    utils::{Core, Response},
};

impl ErrorLogs<Core> {
    pub async fn insert_log_core(self) -> Result<Model, Response> {
        let env = self.environment;

        self.next_phase()
            .insert_log_data(&DBC::db(&env).await?)
            .await
    }
}
