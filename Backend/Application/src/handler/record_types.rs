use crate::{handler::DBC, utils::Response};
use models::{entities::record_types::Model, structs::Environments};

#[derive(Debug, Default)]
pub struct RecordTypes {}

impl RecordTypes {
    pub async fn select(environment: Environments) -> Result<Vec<Model>, Response> {
        let db = DBC::db(&environment).await?;

        Self::select_record_types(&db).await
    }
}
