use chrono::Local;
use function_name::named;
use models::{
    entities::critical_metrics::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::QueryOptions,
};
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

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
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if self.model.id > 0 {
            condition = condition.add(Column::Id.eq(self.model.id));
        }

        let mut stmt = Entity::find().filter(condition);

        stmt = stmt.order_by(Column::Id, sea_orm::Order::Desc);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.as_deref().and_then(Self::parse_order_column) {
                let direction = match q.order_direction.unwrap_or(OrderDirection::Desc) {
                    OrderDirection::Asc => sea_orm::Order::Asc,
                    OrderDirection::Desc => sea_orm::Order::Desc,
                };

                stmt = stmt.order_by(order_by, direction);
            }
        }

        match stmt.all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    fn parse_order_column(value: &str) -> Option<Column> {
        match value {
            "id" => Some(Column::Id),
            "creation_date" => Some(Column::CreationDate),
            "executions_ok" => Some(Column::ExecutionsOk),
            "executions_err" => Some(Column::ExecutionsErr),
            "total_execution_time" => Some(Column::TotalExecutionTime),
            "max_execution_time" => Some(Column::MaxExecutionTime),
            "slowest_duration" => Some(Column::SlowestDuration),
            "active_posting" => Some(Column::ActivePosting),
            "max_active_posting" => Some(Column::MaxActivePosting),
            "skipped_due_to_lock" => Some(Column::SkippedDueToLock),
            "last_success" => Some(Column::LastSuccess),
            "last_error" => Some(Column::LastError),
            "consecutive_errors" => Some(Column::ConsecutiveErrors),
            _ => None,
        }
    }
}
