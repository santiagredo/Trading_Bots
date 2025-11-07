use function_name::named;
use models::{
    entities::status::{Entity, Model},
    structs::ErrorLogRequest,
};
use sea_orm::{DatabaseConnection, EntityTrait};

use crate::{
    handler::{ErrorLogs, OrderStatus},
    log_db_error,
    utils::{Data, Response},
};

impl OrderStatus<Data> {
    #[named]
    pub async fn select_status_data(self, db: &DatabaseConnection) -> Result<Vec<Model>, Response> {
        match Entity::find().all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }
}
