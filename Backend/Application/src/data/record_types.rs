use models::entities::record_types::{Column, Entity, Model};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error_span;

use crate::{
    types::RecordTypes,
    utils::{handle_db_error, Data, Response},
};

impl RecordTypes<Data> {
    pub async fn select_record_types_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(name) = self.model.name {
            condition = condition.add(Column::Name.eq(name))
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }
}
