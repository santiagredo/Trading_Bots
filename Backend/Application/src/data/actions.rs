use crate::utils::handle_db_error;
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::{
    entities::actions::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{ActionRequest, QueryOptions},
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
impl Insert<ActionRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: ActionRequest) -> Result<Model, Response> {
        let req_for_log = req.clone();
        let now = Local::now().naive_local();

        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            strategy_id: ActiveValue::Set(req.strategy_id.unwrap_or_default()),
            is_active: ActiveValue::Set(req.is_active.unwrap_or_default()),
            is_sell: ActiveValue::Set(req.is_sell.unwrap_or_default()),
            is_quote_asset: ActiveValue::Set(req.is_quote_asset.unwrap_or_default()),
            is_percentage: ActiveValue::Set(req.is_percentage.unwrap_or_default()),
            value: ActiveValue::Set(req.value.unwrap_or_default()),
            pair_id: ActiveValue::Set(req.pair_id.unwrap_or_default()),
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
impl Select<ActionRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: ActionRequest) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(strategy_id) = req.strategy_id {
            condition = condition.add(Column::StrategyId.eq(strategy_id));
        }

        if let Some(pair_id) = req.pair_id {
            condition = condition.add(Column::PairId.eq(pair_id));
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
        req: ActionRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(strategy_id) = req.strategy_id {
            condition = condition.add(Column::StrategyId.eq(strategy_id));
        }

        if let Some(is_active) = req.is_active {
            condition = condition.add(Column::IsActive.eq(is_active));
        }

        if let Some(pair_id) = req.pair_id {
            condition = condition.add(Column::PairId.eq(pair_id));
        }

        let mut stmt = Entity::find().filter(condition);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.and_then(|val| parse_order_column(&val)) {
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
        "strategy_id" => Some(Column::StrategyId),
        "is_active" => Some(Column::IsActive),
        "is_sell" => Some(Column::IsSell),
        "is_quote_asset" => Some(Column::IsQuoteAsset),
        "is_percentage" => Some(Column::IsPercentage),
        "value" => Some(Column::Value),
        "pair_id" => Some(Column::PairId),
        "last_update" => Some(Column::LastUpdate),
        _ => None,
    }
}

#[async_trait]
impl Update<ActionRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: ActionRequest) -> Result<Model, Response> {
        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            last_update: ActiveValue::Set(Local::now().naive_local().into()),
            ..Default::default()
        };

        if let Some(strategy_id) = req.strategy_id {
            active_model.strategy_id = ActiveValue::Set(strategy_id);
        }

        if let Some(is_active) = req.is_active {
            active_model.is_active = ActiveValue::Set(is_active);
        }

        if let Some(is_sell) = req.is_sell {
            active_model.is_sell = ActiveValue::Set(is_sell);
        }

        if let Some(is_quote_asset) = req.is_quote_asset {
            active_model.is_quote_asset = ActiveValue::Set(is_quote_asset);
        }

        if let Some(is_percentage) = req.is_percentage {
            active_model.is_percentage = ActiveValue::Set(is_percentage);
        }

        if let Some(value) = req.value {
            active_model.value = ActiveValue::Set(value);
        }

        if let Some(pair_id) = req.pair_id {
            active_model.pair_id = ActiveValue::Set(pair_id);
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
impl Delete<ActionRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: ActionRequest) -> Result<u64, Response> {
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
impl Insert<ActionRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: ActionRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = Model {
            id: req.id.unwrap_or_default(),
            strategy_id: req.strategy_id.unwrap_or_default(),
            is_active: req.is_active.unwrap_or_default(),
            is_sell: req.is_sell.unwrap_or_default(),
            is_quote_asset: req.is_quote_asset.unwrap_or_default(),
            is_percentage: req.is_percentage.unwrap_or_default(),
            value: req.value.unwrap_or_default(),
            pair_id: req.pair_id.unwrap_or_default(),
            last_update: Local::now().naive_local().into(),
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<ActionRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: ActionRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| m.id == req.id.unwrap_or_default())
            .cloned())
    }

    async fn select_many(
        &self,
        _req: ActionRequest,
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
impl Update<ActionRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: ActionRequest) -> Result<Model, Response> {
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
impl Delete<ActionRequest> for MockRepo<Model> {
    async fn delete(&self, req: ActionRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock delete id".to_string()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
