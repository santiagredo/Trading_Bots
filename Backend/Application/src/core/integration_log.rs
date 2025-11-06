use models::entities::integration_log::Model;

use crate::{
    config::get_config,
    handler::IntegrationLogs,
    utils::{Core, Response},
};

impl IntegrationLogs<Core> {
    pub async fn insert_log_core(self) -> Result<Model, Response> {
        self.next_phase()
            .insert_log_data(&get_config().await.db)
            .await
    }
}
