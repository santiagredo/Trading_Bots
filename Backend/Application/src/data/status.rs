use models::entities::status::{Entity, Model};
use sea_orm::{DatabaseConnection, EntityTrait};
use tracing::error_span;

use crate::{
    handler::OrderStatus,
    utils::{handle_db_error, Data, Response},
};

impl OrderStatus<Data> {
    pub async fn select_status_data(self, db: &DatabaseConnection) -> Result<Vec<Model>, Response> {
        match Entity::find().all(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }
}
