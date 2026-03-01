use crate::utils::handle_db_error;
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::{
    entities::integration_log::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{IntegrationLogRequest, QueryOptions},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

use crate::{
    handler::ErrorLogs,
    log_trait_db_error,
    utils::{DbRepo, Insert, MockRepo, Response, Select},
};

#[async_trait]
impl Insert<IntegrationLogRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: IntegrationLogRequest) -> Result<Model, Response> {
        let req_for_log = req.clone();
        let now = Local::now().naive_local();

        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            creation_date: ActiveValue::Set(req.creation_date.unwrap_or(now).into()),
            integration_name: ActiveValue::Set(req.integration_name.unwrap_or_default()),
            function_name: ActiveValue::Set(req.function_name.unwrap_or_default()),
            url: ActiveValue::Set(req.url.unwrap_or_default()),
            request: ActiveValue::Set(req.request.unwrap_or_default()),
            response: ActiveValue::Set(req.response.unwrap_or_default()),
            status_code: ActiveValue::Set(req.status_code.unwrap_or_default()),
            error_message: ActiveValue::Set(req.error_message.unwrap_or_default()),
            execution_time_ms: ActiveValue::Set(req.execution_time_ms.unwrap_or_default()),
        };

        match active_model.insert(&self.data).await {
            Err(err) => {
                let _ = ErrorLogs::new(self.clone())
                    .insert(log_trait_db_error!(err, req_for_log))
                    .await;

                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val),
        }
    }
}

#[async_trait]
impl Select<IntegrationLogRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: IntegrationLogRequest) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        match Entity::find().filter(condition).one(&self.data).await {
            Err(err) => {
                let _ = ErrorLogs::new(self.clone())
                    .insert(log_trait_db_error!(err, req))
                    .await;

                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val),
        }
    }

    #[named]
    async fn select_many(
        &self,
        req: IntegrationLogRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(integration_name) = req.integration_name.as_ref() {
            condition = condition.add(Column::IntegrationName.eq(integration_name.clone()));
        }

        if let Some(function_name) = req.function_name.as_ref() {
            condition = condition.add(Column::FunctionName.eq(function_name.clone()));
        }

        if let Some(status_code) = req.status_code {
            condition = condition.add(Column::StatusCode.eq(status_code));
        }

        let mut stmt = Entity::find()
            .limit(20)
            .order_by(Column::Id, sea_orm::Order::Desc)
            .filter(condition);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.and_then(|v| parse_order_column(&v)) {
                let direction = match q.order_direction.unwrap_or(OrderDirection::Desc) {
                    OrderDirection::Asc => sea_orm::Order::Asc,
                    OrderDirection::Desc => sea_orm::Order::Desc,
                };

                stmt = stmt.order_by(order_by, direction);
            }
        }

        match stmt.all(&self.data).await {
            Err(err) => {
                let _ = ErrorLogs::new(self.clone())
                    .insert(log_trait_db_error!(err, req))
                    .await;

                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val),
        }
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

use crate::utils::Delete;
use crate::utils::Update;

#[async_trait]
impl Update<IntegrationLogRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: IntegrationLogRequest) -> Result<Model, Response> {
        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            ..Default::default()
        };

        if let Some(creation_date) = req.creation_date {
            active_model.creation_date = ActiveValue::Set(creation_date.into());
        }

        if let Some(integration_name) = req.integration_name.as_ref() {
            active_model.integration_name = ActiveValue::Set(integration_name.clone());
        }

        if let Some(function_name) = req.function_name.as_ref() {
            active_model.function_name = ActiveValue::Set(function_name.clone());
        }

        if let Some(url) = req.url.as_ref() {
            active_model.url = ActiveValue::Set(url.clone());
        }

        if let Some(request) = req.request.as_ref() {
            active_model.request = ActiveValue::Set(request.clone());
        }

        if let Some(response) = req.response.as_ref() {
            active_model.response = ActiveValue::Set(response.clone());
        }

        if let Some(status_code) = req.status_code {
            active_model.status_code = ActiveValue::Set(status_code);
        }

        if let Some(error_message) = req.error_message.as_ref() {
            active_model.error_message = ActiveValue::Set(error_message.clone());
        }

        if let Some(execution_time_ms) = req.execution_time_ms {
            active_model.execution_time_ms = ActiveValue::Set(execution_time_ms);
        }

        match active_model.update(&self.data).await {
            Err(err) => {
                let _ = ErrorLogs::new(self.clone())
                    .insert(log_trait_db_error!(err, req))
                    .await;

                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val),
        }
    }
}

#[async_trait]
impl Delete<IntegrationLogRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: IntegrationLogRequest) -> Result<u64, Response> {
        match Entity::delete_by_id(req.id.unwrap_or_default())
            .exec(&self.data)
            .await
        {
            Err(err) => {
                let _ = ErrorLogs::new(self.clone())
                    .insert(log_trait_db_error!(err, req))
                    .await;

                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val.rows_affected),
        }
    }
}

#[async_trait]
impl Insert<IntegrationLogRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: IntegrationLogRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = Model {
            id: req.id.unwrap_or_else(|| (data.len() as i32) + 1),
            creation_date: req
                .creation_date
                .unwrap_or_else(|| Local::now().naive_local())
                .into(),
            integration_name: req.integration_name.unwrap_or_default(),
            function_name: req.function_name.unwrap_or_default(),
            url: req.url.unwrap_or_default(),
            request: req.request.unwrap_or_default(),
            response: req.response.unwrap_or_default(),
            status_code: req.status_code.unwrap_or_default(),
            error_message: req.error_message.unwrap_or_default(),
            execution_time_ms: req.execution_time_ms.unwrap_or_default(),
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<IntegrationLogRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: IntegrationLogRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| m.id == req.id.unwrap_or_default())
            .cloned())
    }

    async fn select_many(
        &self,
        _req: IntegrationLogRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let data = self.data.lock().unwrap();
        let mut result = data.clone();

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                result.truncate(limit as usize);
            }

            if let Some(offset) = q.offset {
                let offset = offset as usize;
                if offset < result.len() {
                    result = result.split_off(offset);
                } else {
                    result.clear();
                }
            }
        }

        Ok(result)
    }
}

#[async_trait]
impl Update<IntegrationLogRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: IntegrationLogRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        let model = data
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        if let Some(creation_date) = req.creation_date {
            model.creation_date = creation_date.into();
        }

        if let Some(integration_name) = req.integration_name {
            model.integration_name = integration_name;
        }

        if let Some(function_name) = req.function_name {
            model.function_name = function_name;
        }

        if let Some(url) = req.url {
            model.url = url;
        }

        if let Some(request) = req.request {
            model.request = request;
        }

        if let Some(response) = req.response {
            model.response = response;
        }

        if let Some(status_code) = req.status_code {
            model.status_code = status_code;
        }

        if let Some(error_message) = req.error_message {
            model.error_message = error_message;
        }

        if let Some(execution_time_ms) = req.execution_time_ms {
            model.execution_time_ms = execution_time_ms;
        }

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<IntegrationLogRequest> for MockRepo<Model> {
    async fn delete(&self, req: IntegrationLogRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock delete id".to_string()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
