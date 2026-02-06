use crate::handler::ErrorLogs;
use crate::utils::handle_db_error;
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::{
    entities::assets::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{AssetRequest, QueryOptions},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

use crate::{
    log_trait_db_error,
    utils::{DbRepo, Delete, Insert, MockRepo, Response, Select, Update},
};

#[async_trait]
impl Insert<AssetRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: AssetRequest) -> Result<Model, Response> {
        let req_for_log = req.clone();

        let now = Local::now().naive_local();

        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(req.name.unwrap_or_default()),
            ticker: ActiveValue::Set(req.ticker.unwrap_or_default()),
            free: ActiveValue::Set(req.free.unwrap_or_default()),
            locked: ActiveValue::Set(req.locked.unwrap_or_default()),
            last_update: ActiveValue::Set(now.into()),
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
impl Select<AssetRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: AssetRequest) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(name) = req.name.as_ref() {
            condition = condition.add(Column::Name.eq(name.clone()));
        }

        if let Some(ticker) = req.ticker.as_ref() {
            condition = condition.add(Column::Ticker.eq(ticker.clone()));
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
        req: AssetRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(name) = req.name.clone() {
            condition = condition.add(Column::Name.eq(name));
        }

        if let Some(ticker) = req.ticker.clone() {
            condition = condition.add(Column::Ticker.eq(ticker));
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
        "ticker" => Some(Column::Ticker),
        "free" => Some(Column::Free),
        "locked" => Some(Column::Locked),
        "last_update" => Some(Column::LastUpdate),
        _ => None,
    }
}

#[async_trait]
impl Update<AssetRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: AssetRequest) -> Result<Model, Response> {
        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            last_update: ActiveValue::Set(Local::now().naive_local().into()),
            ..Default::default()
        };

        if let Some(name) = req.name.as_ref() {
            active_model.name = ActiveValue::Set(name.clone());
        }

        if let Some(ticker) = req.ticker.as_ref() {
            active_model.ticker = ActiveValue::Set(ticker.clone());
        }

        if let Some(free) = req.free {
            active_model.free = ActiveValue::Set(free);
        }

        if let Some(locked) = req.locked {
            active_model.locked = ActiveValue::Set(locked);
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
impl Delete<AssetRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: AssetRequest) -> Result<u64, Response> {
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
impl Insert<AssetRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: AssetRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = Model {
            id: req.id.unwrap_or_default(),
            name: req.name.unwrap_or_default(),
            ticker: req.ticker.unwrap_or_default(),
            free: req.free.unwrap_or_default(),
            locked: req.locked.unwrap_or_default(),
            last_update: Local::now().naive_local().into(),
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<AssetRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: AssetRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| m.id == req.id.unwrap_or_default())
            .cloned())
    }

    async fn select_many(
        &self,
        _req: AssetRequest,
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
impl Update<AssetRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: AssetRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        let model = data
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        if let Some(name) = req.name {
            model.name = name;
        }

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<AssetRequest> for MockRepo<Model> {
    async fn delete(&self, req: AssetRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock delete id".to_string()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
