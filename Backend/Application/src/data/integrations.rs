use crate::utils::handle_db_error;
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::{
    entities::integrations::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{IntegrationRequest, QueryOptions},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

use crate::{
    handler::ErrorLogs,
    log_trait_db_error,
    utils::{DbRepo, Delete, Insert, MockRepo, Response, Select, Update},
};

#[async_trait]
impl Insert<IntegrationRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: IntegrationRequest) -> Result<Model, Response> {
        let req_for_log = req.clone();

        let now = Local::now().naive_local();

        let active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            name: ActiveValue::Set(req.name.unwrap_or_default()),
            code: ActiveValue::Set(req.code.unwrap_or_default()),
            is_enabled: ActiveValue::Set(req.is_enabled.unwrap_or_default()),
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
impl Select<IntegrationRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: IntegrationRequest) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(name) = req.name.clone() {
            condition = condition.add(Column::Name.eq(name));
        }

        if let Some(code) = req.code.clone() {
            condition = condition.add(Column::Code.eq(code));
        }

        if let Some(is_enabled) = req.is_enabled {
            condition = condition.add(Column::IsEnabled.eq(is_enabled));
        }

        let stmt = Entity::find().filter(condition);

        match stmt.one(&self.data).await {
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
        req: IntegrationRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(name) = req.name.clone() {
            condition = condition.add(Column::Name.eq(name));
        }

        if let Some(code) = req.code.clone() {
            condition = condition.add(Column::Code.eq(code));
        }

        if let Some(is_enabled) = req.is_enabled {
            condition = condition.add(Column::IsEnabled.eq(is_enabled));
        }

        let mut stmt = Entity::find().filter(condition);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.and_then(|c| parse_order_column(&c)) {
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
        "name" => Some(Column::Name),
        "code" => Some(Column::Code),
        "is_enabled" => Some(Column::IsEnabled),
        "creation_date" => Some(Column::CreationDate),
        "last_update_date" => Some(Column::LastUpdateDate),
        _ => None,
    }
}

#[async_trait]
impl Update<IntegrationRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: IntegrationRequest) -> Result<Model, Response> {
        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            last_update_date: ActiveValue::Set(Local::now().naive_local().into()),
            ..Default::default()
        };

        if let Some(is_enabled) = req.is_enabled {
            active_model.is_enabled = ActiveValue::Set(is_enabled);
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
impl Delete<IntegrationRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: IntegrationRequest) -> Result<u64, Response> {
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
impl Insert<IntegrationRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: IntegrationRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = Model {
            id: req.id.unwrap_or_default(),
            ..Default::default()
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<IntegrationRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: IntegrationRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| m.id == req.id.unwrap_or_default())
            .cloned())
    }

    async fn select_many(
        &self,
        _req: IntegrationRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let data = self.data.lock().unwrap();

        let mut result = data.clone();

        if let Some(query) = query {
            if let Some(limit) = query.limit {
                result.truncate(limit.try_into().unwrap());
            }
        }

        Ok(result)
    }
}

#[async_trait]
impl Update<IntegrationRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: IntegrationRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        let model = data
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        model.id = id;

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<IntegrationRequest> for MockRepo<Model> {
    async fn delete(&self, req: IntegrationRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock delete id".to_string()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
