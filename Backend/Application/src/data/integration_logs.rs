use chrono::Local;
use function_name::named;
use models::{
    entities::integration_log::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{ErrorLogRequest, QueryOptions},
};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
use tracing::error_span;

use crate::{
    handler::{ErrorLogs, IntegrationLogs},
    log_db_error,
    utils::{handle_db_error, Data, Response},
};

impl IntegrationLogs<Data> {
    pub async fn insert_log_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let now = Local::now().naive_local();

        let active_model_integration_log = ActiveModel {
            id: ActiveValue::NotSet,
            creation_date: ActiveValue::Set(now.into()),
            integration_name: ActiveValue::Set(
                self.model.integration_name.clone().unwrap_or_default(),
            ),
            function_name: ActiveValue::Set(self.model.function_name.clone().unwrap_or_default()),
            url: ActiveValue::Set(self.model.url.clone().unwrap_or_default()),
            request: ActiveValue::Set(self.model.request.clone().unwrap_or_default()),
            response: ActiveValue::Set(self.model.response.clone().unwrap_or_default()),
            status_code: ActiveValue::Set(self.model.status_code.unwrap_or_default()),
            error_message: ActiveValue::Set(self.model.error_message.unwrap_or_default()),
            execution_time_ms: ActiveValue::Set(self.model.execution_time_ms.unwrap_or_default()),
        };

        match Entity::insert(active_model_integration_log)
            .exec_with_returning(db)
            .await
        {
            Err(err) => {
                error_span!(
                    "Error - Database",
                    error = %err,
                    integration_name = %self.model.integration_name.as_deref().unwrap_or(""),
                    function_name = %self.model.function_name.as_deref().unwrap_or(""),
                    url = %self.model.url.as_deref().unwrap_or(""),
                    request = %self.model.request.as_deref().unwrap_or(""),
                    response = %self.model.response.as_deref().unwrap_or(""),
                    status_code = %self.model.status_code.unwrap_or(-1),
                    execution_time_ms = %self.model.execution_time_ms.unwrap_or(-1),
                )
                .in_scope(|| {
                    tracing::error!("Failed to insert in Database");
                });

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_logs_data(
        self,
        db: &DatabaseConnection,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut stmt = Entity::find();

        stmt = stmt.limit(20).order_by(Column::Id, sea_orm::Order::Desc);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.and_then(|c| Self::parse_order_column(&c)) {
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
            "integration_name" => Some(Column::IntegrationName),
            "function_name" => Some(Column::FunctionName),
            "url" => Some(Column::Url),
            "request" => Some(Column::Request),
            "response" => Some(Column::Response),
            "status_code" => Some(Column::StatusCode),
            "error_message" => Some(Column::ErrorMessage),
            "execution_time_ms" => Some(Column::ExecutionTimeMs),
            _ => None,
        }
    }
}
