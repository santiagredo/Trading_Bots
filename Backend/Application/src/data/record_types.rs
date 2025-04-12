use models::entities::record_types::{Entity, Model};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{
    types::RecordTypes,
    utils::{Data, Outcome, OutcomeError},
};

impl RecordTypes<Data> {
    pub async fn select_record_types(
        db: &DatabaseConnection,
    ) -> Outcome<Vec<Model>, String, String> {
        let assets = Entity::find()
            .all(db)
            .await
            .map_err(|err| OutcomeError::Error(err.to_string()))?;

        Outcome::Ok(assets)
    }
}
