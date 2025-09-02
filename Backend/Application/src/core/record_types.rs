use models::entities::record_types::Model;

use crate::{
    config::get_config,
    handler::RecordTypes,
    utils::{Core, Data, Response},
};

impl RecordTypes<Core> {
    pub async fn select_record_types_core(self) -> Result<Vec<Model>, Response> {
        self.next_phase::<Data>()
            .select_record_types_data(&get_config().await.db)
            .await
    }
}
