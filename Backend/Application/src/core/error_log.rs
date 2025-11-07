use models::entities::error_log::Model;

use crate::{
    config::get_config,
    handler::ErrorLogs,
    utils::{Core, Response},
};

impl ErrorLogs<Core> {
    pub async fn insert_log_core(self) -> Result<Model, Response> {
        self.next_phase()
            .insert_log_data(&get_config().await.db)
            .await
    }
}
