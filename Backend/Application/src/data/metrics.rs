use chrono::Local;
use function_name::named;
use models::entities::critical_metrics::{ActiveModel, Column, Entity, Model};
use sea_orm::{ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use models::structs::ErrorLogRequest;

use crate::{
    handler::{ErrorLogs, Metrics},
    log_db_error,
    utils::{Data, Response},
};

impl Metrics<Data> {
    #[named]
    pub async fn insert_metrics_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let now = Local::now().naive_local();

        let active_model_metric = ActiveModel {
            id: ActiveValue::NotSet,
            creation_date: ActiveValue::Set(now.into()),
            executions_ok: ActiveValue::Set(
                self.model.executions_ok.try_into().unwrap_or_default(),
            ),
            executions_err: ActiveValue::Set(
                self.model.executions_err.try_into().unwrap_or_default(),
            ),
            total_execution_time: ActiveValue::Set(
                self.model
                    .total_execution_time
                    .as_millis()
                    .try_into()
                    .unwrap_or_default(),
            ),
            max_execution_time: ActiveValue::Set(
                self.model
                    .max_execution_time
                    .as_millis()
                    .try_into()
                    .unwrap_or_default(),
            ),
            slowest_duration: ActiveValue::Set(
                self.model
                    .slowest_duration
                    .as_millis()
                    .try_into()
                    .unwrap_or_default(),
            ),
            active_posting: ActiveValue::Set(
                self.model.active_posting.try_into().unwrap_or_default(),
            ),
            max_active_posting: ActiveValue::Set(
                self.model.max_posting.try_into().unwrap_or_default(),
            ),
            skipped_due_to_lock: ActiveValue::Set(
                self.model
                    .skipped_due_to_lock
                    .try_into()
                    .unwrap_or_default(),
            ),
            last_success: ActiveValue::Set(self.model.last_success),
            last_error: ActiveValue::Set(self.model.last_error),
            consecutive_errors: ActiveValue::Set(
                self.model.consecutive_errors.try_into().unwrap_or_default(),
            ),
        };

        match Entity::insert(active_model_metric)
            .exec_with_returning(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_metrics_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if self.model.id > 0 {
            condition = condition.add(Column::Id.eq(self.model.id))
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }
}
