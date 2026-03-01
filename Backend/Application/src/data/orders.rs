use crate::utils::handle_db_error;
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::{
    entities::orders::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{OrderRequest, QueryOptions},
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

/* =========================================================
 * DB REPO
 * ========================================================= */

#[async_trait]
impl Insert<OrderRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: OrderRequest) -> Result<Model, Response> {
        let req_for_log = req.clone();
        let now = Local::now().naive_local();

        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            status_id: ActiveValue::Set(req.status_id.unwrap_or_default()),
            creation_date: ActiveValue::Set(now.into()),
            update_date: ActiveValue::Set(now.into()),
            is_sell: ActiveValue::Set(req.is_sell.unwrap_or_default()),
            strategy_id: ActiveValue::Set(req.strategy_id.unwrap_or_default()),
            base_asset_id: ActiveValue::Set(req.base_asset_id.unwrap_or_default()),
            base_asset_amount: ActiveValue::Set(req.base_asset_amount.unwrap_or_default()),
            quote_asset_id: ActiveValue::Set(req.quote_asset_id.unwrap_or_default()),
            quote_asset_amount: ActiveValue::Set(req.quote_asset_amount.unwrap_or_default()),
            price_entry: ActiveValue::Set(req.price_entry.unwrap_or_default()),
            price_target: ActiveValue::Set(req.price_target.unwrap_or_default()),
            price_abort: ActiveValue::Set(req.price_abort.unwrap_or_default()),
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
impl Select<OrderRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: OrderRequest) -> Result<Option<Model>, Response> {
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
        req: OrderRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(status_id) = req.status_id {
            condition = condition.add(Column::StatusId.eq(status_id));
        }

        if let Some(is_sell) = req.is_sell {
            condition = condition.add(Column::IsSell.eq(is_sell));
        }

        if let Some(strategy_id) = req.strategy_id {
            condition = condition.add(Column::StrategyId.eq(strategy_id));
        }

        if let Some(base_asset_id) = req.base_asset_id {
            condition = condition.add(Column::BaseAssetId.eq(base_asset_id));
        }

        if let Some(quote_asset_id) = req.quote_asset_id {
            condition = condition.add(Column::QuoteAssetId.eq(quote_asset_id));
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
        "status_id" => Some(Column::StatusId),
        "creation_date" => Some(Column::CreationDate),
        "update_date" => Some(Column::UpdateDate),
        "is_sell" => Some(Column::IsSell),
        "strategy_id" => Some(Column::StrategyId),
        "base_asset_id" => Some(Column::BaseAssetId),
        "base_asset_amount" => Some(Column::BaseAssetAmount),
        "quote_asset_id" => Some(Column::QuoteAssetId),
        "quote_asset_amount" => Some(Column::QuoteAssetAmount),
        "price_entry" => Some(Column::PriceEntry),
        "price_target" => Some(Column::PriceTarget),
        "price_abort" => Some(Column::PriceAbort),
        _ => None,
    }
}

#[async_trait]
impl Update<OrderRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: OrderRequest) -> Result<Model, Response> {
        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(req.id.unwrap_or_default()),
            update_date: ActiveValue::Set(Local::now().naive_local().into()),
            ..Default::default()
        };

        if let Some(status_id) = req.status_id {
            active_model.status_id = ActiveValue::Set(status_id);
        }

        if let Some(base_asset_amount) = req.base_asset_amount {
            active_model.base_asset_amount = ActiveValue::Set(base_asset_amount);
        }

        if let Some(quote_asset_amount) = req.quote_asset_amount {
            active_model.quote_asset_amount = ActiveValue::Set(quote_asset_amount);
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
impl Delete<OrderRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: OrderRequest) -> Result<u64, Response> {
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

/* =========================================================
 * MOCK REPO
 * ========================================================= */

#[async_trait]
impl Insert<OrderRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: OrderRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = Model {
            id: req.id.unwrap_or_default(),
            status_id: req.status_id.unwrap_or_default(),
            creation_date: Local::now().naive_local().into(),
            update_date: Local::now().naive_local().into(),
            is_sell: req.is_sell.unwrap_or_default(),
            strategy_id: req.strategy_id.unwrap_or_default(),
            base_asset_id: req.base_asset_id.unwrap_or_default(),
            base_asset_amount: req.base_asset_amount.unwrap_or_default(),
            quote_asset_id: req.quote_asset_id.unwrap_or_default(),
            quote_asset_amount: req.quote_asset_amount.unwrap_or_default(),
            price_entry: req.price_entry.unwrap_or_default(),
            price_target: req.price_target.unwrap_or_default(),
            price_abort: req.price_abort.unwrap_or_default(),
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<OrderRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: OrderRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| m.id == req.id.unwrap_or_default())
            .cloned())
    }

    async fn select_many(
        &self,
        _req: OrderRequest,
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
impl Update<OrderRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: OrderRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        let model = data
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        if let Some(status_id) = req.status_id {
            model.status_id = status_id;
        }

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<OrderRequest> for MockRepo<Model> {
    async fn delete(&self, req: OrderRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock delete id".to_string()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
