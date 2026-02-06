use crate::handler::OrderStatus;
use crate::utils::handle_db_error;
use crate::utils::Response;
use models::entities::status::{Entity, Model};
use sea_orm::{DatabaseConnection, EntityTrait};

impl OrderStatus {
    // #[named]
    pub async fn select_status_data(db: &DatabaseConnection) -> Result<Vec<Model>, Response> {
        match Entity::find().all(db).await {
            Err(err) => {
                // let _ = ErrorLogs::new(self.clone())
                //     .insert(log_trait_db_error!(err, format!("Empty request")))
                //     .await;

                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val),
        }
    }
}
