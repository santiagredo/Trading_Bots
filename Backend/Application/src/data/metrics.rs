use crate::utils::{handle_db_error, Delete, MockRepo, Update};
use crate::{
    handler::ErrorLogs,
    log_trait_db_error,
    utils::{DbRepo, Insert, Response, Select},
};
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::structs::MetricRequest;
use models::{
    entities::critical_metrics::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::QueryOptions,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

#[async_trait]
impl Insert<MetricRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: MetricRequest) -> Result<Model, Response> {
        let req_for_log = req.clone();
        let now = Local::now().naive_local();

        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            creation_date: ActiveValue::Set(now.into()),
            executions_ok: ActiveValue::Set(req.executions_ok.unwrap_or_default()),
            executions_err: ActiveValue::Set(req.executions_err.unwrap_or_default()),
            total_execution_time: ActiveValue::Set(req.total_execution_time.unwrap_or_default()),
            max_execution_time: ActiveValue::Set(req.max_execution_time.unwrap_or_default()),
            slowest_duration: ActiveValue::Set(req.slowest_duration.unwrap_or_default()),
            active_posting: ActiveValue::Set(req.active_posting.unwrap_or_default()),
            max_active_posting: ActiveValue::Set(req.max_active_posting.unwrap_or_default()),
            skipped_due_to_lock: ActiveValue::Set(req.skipped_due_to_lock.unwrap_or_default()),
            last_success: ActiveValue::Set(req.last_success),
            last_error: ActiveValue::Set(req.last_error),
            consecutive_errors: ActiveValue::Set(req.consecutive_errors.unwrap_or_default()),
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
impl Select<MetricRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: MetricRequest) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if req.id.is_some_and(|id| id > 0) {
            condition = condition.add(Column::Id.eq(req.id));
        }

        match Entity::find().filter(condition).one(&self.data).await {
            Err(err) => {
                let _ = ErrorLogs::new(self.clone())
                    .insert(log_trait_db_error!(err, "select critical_metrics"))
                    .await;

                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val),
        }
    }

    #[named]
    async fn select_many(
        &self,
        req: MetricRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if req.id.is_some_and(|id| id > 0) {
            condition = condition.add(Column::Id.eq(req.id));
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

            if let Some(order_by) = q.order_by.and_then(|val| parse_order_column(&val)) {
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
                    .insert(log_trait_db_error!(err, "select_many critical_metrics"))
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

#[async_trait]
impl Update<MetricRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: MetricRequest) -> Result<Model, Response> {
        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            ..Default::default()
        };

        if let Some(val) = req.executions_ok {
            active_model.executions_ok = ActiveValue::Set(val);
        }

        if let Some(val) = req.executions_err {
            active_model.executions_err = ActiveValue::Set(val);
        }

        if let Some(val) = req.total_execution_time {
            active_model.total_execution_time = ActiveValue::Set(val);
        }

        if let Some(val) = req.max_execution_time {
            active_model.max_execution_time = ActiveValue::Set(val);
        }

        if let Some(val) = req.slowest_duration {
            active_model.slowest_duration = ActiveValue::Set(val);
        }

        if let Some(val) = req.active_posting {
            active_model.active_posting = ActiveValue::Set(val);
        }

        if let Some(val) = req.max_active_posting {
            active_model.max_active_posting = ActiveValue::Set(val);
        }

        if let Some(val) = req.skipped_due_to_lock {
            active_model.skipped_due_to_lock = ActiveValue::Set(val);
        }

        if let Some(val) = req.last_success {
            active_model.last_success = ActiveValue::Set(Some(val));
        }

        if let Some(val) = req.last_error {
            active_model.last_error = ActiveValue::Set(Some(val));
        }

        if let Some(val) = req.consecutive_errors {
            active_model.consecutive_errors = ActiveValue::Set(val);
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
impl Delete<MetricRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: MetricRequest) -> Result<u64, Response> {
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
impl Insert<MetricRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: MetricRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let mut model = Model {
            id: req.id.unwrap_or(0),
            creation_date: req
                .creation_date
                .unwrap_or_else(|| Local::now().naive_local().into()),
            executions_ok: req.executions_ok.unwrap_or(0),
            executions_err: req.executions_err.unwrap_or(0),
            total_execution_time: req.total_execution_time.unwrap_or(0),
            max_execution_time: req.max_execution_time.unwrap_or(0),
            slowest_duration: req.slowest_duration.unwrap_or(0),
            active_posting: req.active_posting.unwrap_or(0),
            max_active_posting: req.max_active_posting.unwrap_or(0),
            skipped_due_to_lock: req.skipped_due_to_lock.unwrap_or(0),
            last_success: req.last_success,
            last_error: req.last_error,
            consecutive_errors: req.consecutive_errors.unwrap_or(0),
        };

        // Simula autoincrement
        if model.id == 0 {
            model.id = (data.len() as i32) + 1;
        }

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<MetricRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: MetricRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(match req.id {
            Some(id) => data.iter().find(|m| m.id == id).cloned(),
            None => None,
        })
    }

    async fn select_many(
        &self,
        _req: MetricRequest,
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
impl Update<MetricRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: MetricRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req.id.ok_or(Response::not_found(
            "Missing id for mock update".to_string(),
        ))?;

        let model = data
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        if let Some(v) = req.executions_ok {
            model.executions_ok = v;
        }
        if let Some(v) = req.executions_err {
            model.executions_err = v;
        }
        if let Some(v) = req.total_execution_time {
            model.total_execution_time = v;
        }
        if let Some(v) = req.max_execution_time {
            model.max_execution_time = v;
        }
        if let Some(v) = req.slowest_duration {
            model.slowest_duration = v;
        }
        if let Some(v) = req.active_posting {
            model.active_posting = v;
        }
        if let Some(v) = req.max_active_posting {
            model.max_active_posting = v;
        }
        if let Some(v) = req.skipped_due_to_lock {
            model.skipped_due_to_lock = v;
        }
        if let Some(v) = req.last_success {
            model.last_success = Some(v);
        }
        if let Some(v) = req.last_error {
            model.last_error = Some(v);
        }
        if let Some(v) = req.consecutive_errors {
            model.consecutive_errors = v;
        }

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<MetricRequest> for MockRepo<Model> {
    async fn delete(&self, req: MetricRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let before = data.len();
        data.retain(|m| m.id != req.id.unwrap_or_default());

        Ok((before - data.len()) as u64)
    }
}
