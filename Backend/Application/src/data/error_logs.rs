use crate::utils::{handle_db_error, DbRepo, Delete, Insert, MockRepo, Response, Select, Update};
use chrono::Local;
use migration::async_trait::async_trait;
use models::{
    entities::error_log::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{ErrorLogRequest, QueryOptions},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};
use tracing::error_span;

#[async_trait]
impl Insert<ErrorLogRequest, Model> for DbRepo {
    async fn insert(&self, req: ErrorLogRequest) -> Result<Model, Response> {
        let now = Local::now().naive_local();

        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            creation_date: ActiveValue::Set(now.into()),
            file_path: ActiveValue::Set(req.file_path.clone().unwrap_or_default()),
            line_number: ActiveValue::Set(req.line_number.clone().unwrap_or_default()),
            function_name: ActiveValue::Set(req.function_name.clone().unwrap_or_default()),
            request: ActiveValue::Set(req.request.clone().unwrap_or_default()),
            error_type: ActiveValue::Set(req.error_type.clone().unwrap_or_default()),
            error_details: ActiveValue::Set(req.error_details.clone().unwrap_or_default()),
        };

        match active_model.insert(&self.data).await {
            Err(err) => {
                error_span!(
                    "Error - Database",
                    error = %err,
                    file_path = %req.file_path.as_deref().unwrap_or(""),
                    line_number = %req.line_number.as_deref().unwrap_or(""),
                    function_name = %req.function_name.as_deref().unwrap_or(""),
                    request = %req.request.as_deref().unwrap_or(""),
                    error_type = %req.error_type.as_deref().unwrap_or(""),
                    error_details = %req.error_details.as_deref().unwrap_or(""),
                )
                .in_scope(|| {
                    tracing::error!("Failed to insert ErrorLog in Database");
                });

                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val),
        }
    }
}

#[async_trait]
impl Select<ErrorLogRequest, Model> for DbRepo {
    async fn select(&self, req: ErrorLogRequest) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        match Entity::find().filter(condition).one(&self.data).await {
            Err(err) => Err(handle_db_error(&err)),
            Ok(val) => Ok(val),
        }
    }

    async fn select_many(
        &self,
        _req: ErrorLogRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut stmt = Entity::find()
            .limit(20)
            .order_by(Column::Id, sea_orm::Order::Desc);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.and_then(|c| parse_order_column(&c)) {
                let direction = match q.order_direction.unwrap_or(OrderDirection::Desc) {
                    OrderDirection::Asc => sea_orm::Order::Asc,
                    OrderDirection::Desc => sea_orm::Order::Desc,
                };

                stmt = stmt.order_by(order_by, direction);
            }
        }

        match stmt.all(&self.data).await {
            Err(err) => Err(handle_db_error(&err)),
            Ok(val) => Ok(val),
        }
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

#[async_trait]
impl Update<ErrorLogRequest, Model> for DbRepo {
    async fn update(&self, req: ErrorLogRequest) -> Result<Model, Response> {
        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            ..Default::default()
        };

        if let Some(error_details) = req.error_details.clone() {
            active_model.error_details = ActiveValue::Set(error_details);
        }

        match active_model.update(&self.data).await {
            Err(err) => {
                error_span!(
                    "Error - Database",
                    error = %err,
                    id = %req.id.unwrap_or_default(),
                )
                .in_scope(|| {
                    tracing::error!("Failed to update ErrorLog in Database");
                });

                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val),
        }
    }
}

#[async_trait]
impl Delete<ErrorLogRequest> for DbRepo {
    async fn delete(&self, req: ErrorLogRequest) -> Result<u64, Response> {
        match Entity::delete_by_id(req.id.unwrap_or_default())
            .exec(&self.data)
            .await
        {
            Err(err) => Err(handle_db_error(&err)),
            Ok(val) => Ok(val.rows_affected),
        }
    }
}

#[async_trait]
impl Insert<ErrorLogRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: ErrorLogRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = Model {
            id: req.id.unwrap_or_default(),
            creation_date: Local::now().naive_local().into(),
            file_path: req.file_path.unwrap_or_default(),
            line_number: req.line_number.unwrap_or_default(),
            function_name: req.function_name.unwrap_or_default(),
            request: req.request.unwrap_or_default(),
            error_type: req.error_type.unwrap_or_default(),
            error_details: req.error_details.unwrap_or_default(),
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<ErrorLogRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: ErrorLogRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| m.id == req.id.unwrap_or_default())
            .cloned())
    }

    async fn select_many(
        &self,
        _req: ErrorLogRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let data = self.data.lock().unwrap();
        let mut result = data.clone();

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                result.truncate(limit as usize);
            }
        }

        Ok(result)
    }
}

#[async_trait]
impl Update<ErrorLogRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: ErrorLogRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        let model = data
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        if let Some(error_details) = req.error_details {
            model.error_details = error_details;
        }

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<ErrorLogRequest> for MockRepo<Model> {
    async fn delete(&self, req: ErrorLogRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock delete id".to_string()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
