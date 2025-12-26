use models::entities::record_types::Model;

use crate::{
    handler::{RecordTypes, DBC},
    utils::{Core, Data, Response},
};

impl RecordTypes<Core> {
    pub async fn select_record_types_core(self) -> Result<Vec<Model>, Response> {
        let env = self.environment;

        self.next_phase::<Data>()
            .select_record_types_data(&DBC::db(&env).await?)
            .await
    }
}
