use function_name::named;
use models::{
    entities::record_types::{Column, Entity, Model},
    structs::ErrorLogRequest,
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    handler::{ErrorLogs, RecordTypes},
    log_db_error,
    utils::{Data, Response},
};

impl RecordTypes<Data> {
    #[named]
    pub async fn select_record_types_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(name) = self.model.name.as_ref() {
            condition = condition.add(Column::Name.eq(name.clone()))
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }
}
