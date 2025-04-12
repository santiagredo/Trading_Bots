use models::entities::record_types::Model;

use crate::{
    config::get_config,
    types::RecordTypes,
    utils::{Core, Data, Outcome},
};

impl RecordTypes<Core> {
    pub async fn select_record_types() -> Outcome<Vec<Model>, String, String> {
        // RecordTypes::<Logic>::select_record_types();

        RecordTypes::<Data>::select_record_types(&get_config().await.db).await
    }
}
