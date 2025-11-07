use chrono::Local;
use models::entities::error_log::{ActiveModel, Entity, Model};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait};
use tracing::error_span;

use crate::{
    handler::ErrorLogs,
    utils::{handle_db_error, Data, Response},
};

impl ErrorLogs<Data> {
    pub async fn insert_log_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let now = Local::now().naive_local();

        let active_model_error_log = ActiveModel {
            id: ActiveValue::NotSet,
            creation_date: ActiveValue::Set(now.into()),
            file_path: ActiveValue::Set(self.model.file_path.clone().unwrap_or_default()),
            line_number: ActiveValue::Set(self.model.line_number.clone().unwrap_or_default()),
            function_name: ActiveValue::Set(self.model.function_name.clone().unwrap_or_default()),
            request: ActiveValue::Set(self.model.request.clone().unwrap_or_default()),
            error_type: ActiveValue::Set(self.model.error_type.clone().unwrap_or_default()),
            error_details: ActiveValue::Set(self.model.error_details.clone().unwrap_or_default()),
        };

        match Entity::insert(active_model_error_log)
            .exec_with_returning(db)
            .await
        {
            Err(err) => {
                error_span!(
                    "Error - Database",
                    error = %err,
                    file_path = %self.model.file_path.as_deref().unwrap_or(""),
                    line_number = %self.model.line_number.as_deref().unwrap_or(""),
                    function_name = %self.model.function_name.as_deref().unwrap_or(""),
                    request = %self.model.request.as_deref().unwrap_or(""),
                    error_type = %self.model.error_type.as_deref().unwrap_or(""),
                    error_details = %self.model.error_details.as_deref().unwrap_or(""),
                )
                .in_scope(|| {
                    tracing::error!("Failed to insert in Database");
                });

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }
}
