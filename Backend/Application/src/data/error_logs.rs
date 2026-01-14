use chrono::Local;
use function_name::named;
use models::{
    entities::error_log::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{ErrorLogRequest, QueryOptions},
};
use sea_orm::{ActiveValue, DatabaseConnection, EntityTrait, QueryOrder, QuerySelect};
use tracing::error_span;

use crate::{
    handler::ErrorLogs,
    log_db_error,
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
            "file_path" => Some(Column::FilePath),
            "line_number" => Some(Column::LineNumber),
            "function_name" => Some(Column::FunctionName),
            "request" => Some(Column::Request),
            "error_type" => Some(Column::ErrorType),
            "error_details" => Some(Column::ErrorDetails),
            _ => None,
        }
    }
}
