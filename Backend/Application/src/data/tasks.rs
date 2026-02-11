use crate::utils::handle_db_error;
use crate::{
    handler::ErrorLogs,
    log_trait_db_error,
    utils::{DbRepo, Delete, Insert, MockRepo, Response, Select, Update},
};
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::{
    entities::tasks::{self, Column, Entity, Model},
    enums::OrderDirection,
    structs::{QueryOptions, TaskRequest},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

/* ======================================================
 * DB REPO
 * ======================================================
 */

#[async_trait]
impl Insert<TaskRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: TaskRequest) -> Result<Model, Response> {
        let req_for_log = req.clone();

        let active_model = tasks::ActiveModel {
            id: ActiveValue::NotSet,
            nick: ActiveValue::Set(req.nick.unwrap_or_default()),
            description: ActiveValue::Set(req.description.unwrap_or_default()),
            is_active: ActiveValue::Set(req.is_active.unwrap_or(true)),
            cooldown: ActiveValue::Set(req.cooldown.unwrap_or_default().into()),
            delay: ActiveValue::Set(req.delay.unwrap_or_default().into()),
            last_update: ActiveValue::Set(Local::now().naive_local().into()),
            last_execution: ActiveValue::Set(req.last_execution.map(Into::into)),
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
impl Select<TaskRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: TaskRequest) -> Result<Option<Model>, Response> {
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
        req: TaskRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(nick) = req.nick.as_ref() {
            condition = condition.add(Column::Nick.eq(nick));
        }

        if let Some(description) = req.description.as_ref() {
            condition = condition.add(Column::Description.eq(description));
        }

        if let Some(is_active) = req.is_active {
            condition = condition.add(Column::IsActive.eq(is_active));
        }

        if let Some(cooldown) = req.cooldown {
            condition = condition.add(Column::Cooldown.eq(cooldown));
        }

        if let Some(delay) = req.delay {
            condition = condition.add(Column::Delay.eq(delay));
        }

        let mut stmt = Entity::find().filter(condition);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.as_deref().and_then(parse_order_column) {
                let direction = match q.order_direction.unwrap_or(OrderDirection::Asc) {
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
        "nick" => Some(Column::Nick),
        "description" => Some(Column::Description),
        "is_active" => Some(Column::IsActive),
        "cooldown" => Some(Column::Cooldown),
        "delay" => Some(Column::Delay),
        "last_update" => Some(Column::LastUpdate),
        "last_execution" => Some(Column::LastExecution),
        _ => None,
    }
}

#[async_trait]
impl Update<TaskRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: TaskRequest) -> Result<Model, Response> {
        let mut active_model = tasks::ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            last_update: ActiveValue::Set(Local::now().naive_local().into()),
            ..Default::default()
        };

        if let Some(is_active) = req.is_active {
            active_model.is_active = ActiveValue::Set(is_active);
        }

        if let Some(cooldown) = req.cooldown {
            active_model.cooldown = ActiveValue::Set(cooldown.into());
        }

        if let Some(delay) = req.delay {
            active_model.delay = ActiveValue::Set(delay.into());
        }

        if let Some(last_execution) = req.last_execution {
            active_model.last_execution = ActiveValue::Set(last_execution.into());
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
impl Delete<TaskRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: TaskRequest) -> Result<u64, Response> {
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

/* ======================================================
 * MOCK REPO
 * ======================================================
 */

#[async_trait]
impl Insert<TaskRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: TaskRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = Model {
            id: req.id.unwrap_or_default(),
            nick: req.nick.unwrap_or_default(),
            description: req.description.unwrap_or_default(),
            is_active: req.is_active.unwrap_or(true),
            cooldown: req.cooldown.unwrap_or_default(),
            delay: req.delay.unwrap_or_default(),
            last_update: Local::now().naive_local().into(),
            last_execution: req.last_execution.map(Into::into),
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<TaskRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: TaskRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| m.id == req.id.unwrap_or_default())
            .cloned())
    }

    async fn select_many(
        &self,
        _req: TaskRequest,
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
impl Update<TaskRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: TaskRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        let model = data
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        if let Some(is_active) = req.is_active {
            model.is_active = is_active;
        }

        if let Some(cooldown) = req.cooldown {
            model.cooldown = cooldown;
        }

        if let Some(delay) = req.delay {
            model.delay = delay;
        }

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<TaskRequest> for MockRepo<Model> {
    async fn delete(&self, req: TaskRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock delete id".to_string()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
