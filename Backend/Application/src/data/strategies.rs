use crate::utils::handle_db_error;
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::{
    entities::strategies::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{QueryOptions, StrategyRequest},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

use crate::logic;
use crate::utils::handle_user_err;
use crate::{
    handler::ErrorLogs,
    log_trait_db_error,
    utils::{DbRepo, Delete, Insert, MockRepo, Response, Select, Update},
};

#[async_trait]
impl Insert<StrategyRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: StrategyRequest) -> Result<Model, Response> {
        logic::strategies::validate_insert(&req).map_err(handle_user_err)?;

        let req_for_log = req.clone();
        let now = Local::now().naive_local();

        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(req.name.unwrap_or_default()),
            is_active: ActiveValue::Set(req.is_active.unwrap_or_default()),
            description: ActiveValue::Set(req.description),
            can_trade: ActiveValue::Set(req.can_trade.unwrap_or_default()),
            last_execution: ActiveValue::Set(req.last_execution),
            cooldown: ActiveValue::Set(req.cooldown),
            error_cooldown: ActiveValue::Set(req.error_cooldown),
            error_last_date: ActiveValue::Set(req.error_last_date),
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
impl Select<StrategyRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: StrategyRequest) -> Result<Option<Model>, Response> {
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
        req: StrategyRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(name) = req.name.as_ref() {
            condition = condition.add(Column::Name.eq(name));
        }

        if let Some(is_active) = req.is_active {
            condition = condition.add(Column::IsActive.eq(is_active));
        }

        if let Some(can_trade) = req.can_trade {
            condition = condition.add(Column::CanTrade.eq(can_trade));
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
        "name" => Some(Column::Name),
        "is_active" => Some(Column::IsActive),
        "can_trade" => Some(Column::CanTrade),
        "description" => Some(Column::Description),
        "last_execution" => Some(Column::LastExecution),
        "cooldown" => Some(Column::Cooldown),
        "error_cooldown" => Some(Column::ErrorCooldown),
        "error_last_date" => Some(Column::ErrorLastDate),
        "last_update" => Some(Column::LastUpdate),
        _ => None,
    }
}

#[async_trait]
impl Update<StrategyRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: StrategyRequest) -> Result<Model, Response> {
        logic::strategies::validate_update(&req).map_err(handle_user_err)?;

        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            last_update: ActiveValue::Set(Local::now().naive_local().into()),
            ..Default::default()
        };

        if let Some(name) = req.name.as_ref() {
            active_model.name = ActiveValue::Set(name.clone());
        }

        if let Some(is_active) = req.is_active {
            active_model.is_active = ActiveValue::Set(is_active);
        }

        if let Some(can_trade) = req.can_trade {
            active_model.can_trade = ActiveValue::Set(can_trade);
        }

        if let Some(description) = req.description.as_ref() {
            active_model.description = ActiveValue::Set(Some(description.clone()));
        }

        if let Some(last_execution) = req.last_execution {
            active_model.last_execution = ActiveValue::Set(Some(last_execution));
        }

        if let Some(cooldown) = req.cooldown {
            active_model.cooldown = ActiveValue::Set(Some(cooldown));
        }

        if let Some(error_cooldown) = req.error_cooldown {
            active_model.error_cooldown = ActiveValue::Set(Some(error_cooldown));
        }

        if let Some(error_last_date) = req.error_last_date {
            active_model.error_last_date = ActiveValue::Set(Some(error_last_date));
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
impl Delete<StrategyRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: StrategyRequest) -> Result<u64, Response> {
        logic::strategies::validate_delete(&req).map_err(handle_user_err)?;

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
impl Insert<StrategyRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: StrategyRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = Model {
            id: req.id.unwrap_or_default(),
            name: req.name.unwrap_or_default(),
            is_active: req.is_active.unwrap_or_default(),
            description: req.description,
            can_trade: req.can_trade.unwrap_or_default(),
            last_execution: req.last_execution,
            cooldown: req.cooldown,
            error_cooldown: req.error_cooldown,
            error_last_date: req.error_last_date,
            last_update: Local::now().naive_local().into(),
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<StrategyRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: StrategyRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| m.id == req.id.unwrap_or_default())
            .cloned())
    }

    async fn select_many(
        &self,
        _req: StrategyRequest,
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
impl Update<StrategyRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: StrategyRequest) -> Result<Model, Response> {
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

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<StrategyRequest> for MockRepo<Model> {
    async fn delete(&self, req: StrategyRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock delete id".to_string()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
