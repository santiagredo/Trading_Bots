use crate::handler::ErrorLogs;
use crate::utils::handle_db_error;
use crate::{
    log_trait_db_error,
    utils::{DbRepo, Delete, Insert, MockRepo, Response, Select, Update},
};
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::{
    entities::integration_settings::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{IntegrationSettingRequest, QueryOptions},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

#[async_trait]
impl Insert<IntegrationSettingRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: IntegrationSettingRequest) -> Result<Model, Response> {
        let req_for_log = req.clone();
        let now = Local::now().naive_local();

        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            integration_id: ActiveValue::Set(req.integration_id.unwrap_or_default()),
            name: ActiveValue::Set(req.name.unwrap_or_default()),
            nick: ActiveValue::Set(req.nick.unwrap_or_default()),
            value: ActiveValue::Set(req.value.unwrap_or_default()),
            creation_date: ActiveValue::Set(now.into()),
            last_update_date: ActiveValue::Set(now.into()),
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
impl Select<IntegrationSettingRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: IntegrationSettingRequest) -> Result<Option<Model>, Response> {
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
        req: IntegrationSettingRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(integration_id) = req.integration_id {
            condition = condition.add(Column::IntegrationId.eq(integration_id));
        }

        if let Some(name) = req.name.as_ref() {
            condition = condition.add(Column::Name.eq(name.clone()));
        }

        if let Some(nick) = req.nick.as_ref() {
            condition = condition.add(Column::Nick.eq(nick.clone()));
        }

        let mut stmt = Entity::find().filter(condition);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.and_then(|v| parse_order_column(&v)) {
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
        "integration_id" => Some(Column::IntegrationId),
        "name" => Some(Column::Name),
        "nick" => Some(Column::Nick),
        "value" => Some(Column::Value),
        "creation_date" => Some(Column::CreationDate),
        "last_update_date" => Some(Column::LastUpdateDate),
        _ => None,
    }
}

#[async_trait]
impl Update<IntegrationSettingRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: IntegrationSettingRequest) -> Result<Model, Response> {
        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            last_update_date: ActiveValue::Set(Local::now().naive_local().into()),
            ..Default::default()
        };

        if let Some(value) = req.value.as_ref() {
            active_model.value = ActiveValue::Set(value.clone());
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
impl Delete<IntegrationSettingRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: IntegrationSettingRequest) -> Result<u64, Response> {
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
impl Insert<IntegrationSettingRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: IntegrationSettingRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();
        let now = Local::now().naive_local();

        let model = Model {
            id: req.id.unwrap_or_else(|| data.len() as i32 + 1),
            integration_id: req.integration_id.unwrap_or_default(),
            name: req.name.unwrap_or_default(),
            nick: req.nick.unwrap_or_default(),
            value: req.value.unwrap_or_default(),
            creation_date: now.into(),
            last_update_date: now.into(),
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<IntegrationSettingRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: IntegrationSettingRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| m.id == req.id.unwrap_or_default())
            .cloned())
    }

    async fn select_many(
        &self,
        _req: IntegrationSettingRequest,
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
impl Update<IntegrationSettingRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: IntegrationSettingRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock update id".into()))?;

        let model = data
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or(Response::not_found("Invalid mock update id".into()))?;

        if let Some(value) = req.value {
            model.value = value;
        }

        model.last_update_date = Local::now().naive_local().into();

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<IntegrationSettingRequest> for MockRepo<Model> {
    async fn delete(&self, req: IntegrationSettingRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();
        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock delete id".into()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
