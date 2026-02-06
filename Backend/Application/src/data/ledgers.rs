use crate::handler::ErrorLogs;
use crate::utils::handle_db_error;
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::{
    entities::ledgers::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{LedgerRequest, QueryOptions},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

use crate::{
    log_trait_db_error,
    utils::{DbRepo, Delete, Insert, MockRepo, Response, Select, Update},
};

/* =========================================================
 * DbRepo
 * ========================================================= */

#[async_trait]
impl Insert<LedgerRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: LedgerRequest) -> Result<Model, Response> {
        let req = req.clone();

        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            order_id: ActiveValue::Set(req.order_id),
            record_type_id: ActiveValue::Set(req.record_type_id.unwrap_or_default()),
            creation_date: ActiveValue::Set(Local::now().naive_local()),
            asset_id: ActiveValue::Set(req.asset_id.unwrap_or_default()),
            free_amount: ActiveValue::Set(req.free_amount.unwrap_or_default()),
            free_previous_balance: ActiveValue::Set(req.free_previous_balance.unwrap_or_default()),
            free_new_balance: ActiveValue::Set(req.free_new_balance.unwrap_or_default()),
            locked_amount: ActiveValue::Set(req.locked_amount.unwrap_or_default()),
            locked_previous_balance: ActiveValue::Set(
                req.locked_previous_balance.unwrap_or_default(),
            ),
            locked_new_balance: ActiveValue::Set(req.locked_new_balance.unwrap_or_default()),
        };

        match active_model.insert(&self.data).await {
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
impl Select<LedgerRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: LedgerRequest) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(order_id) = req.order_id {
            condition = condition.add(Column::OrderId.eq(order_id));
        }

        if let Some(record_type_id) = req.record_type_id {
            condition = condition.add(Column::RecordTypeId.eq(record_type_id));
        }

        if let Some(asset_id) = req.asset_id {
            condition = condition.add(Column::AssetId.eq(asset_id));
        }

        match Entity::find()
            .filter(condition)
            .order_by(Column::CreationDate, sea_orm::Order::Desc)
            .one(&self.data)
            .await
        {
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
        req: LedgerRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(order_id) = req.order_id {
            condition = condition.add(Column::OrderId.eq(order_id));
        }

        if let Some(record_type_id) = req.record_type_id {
            condition = condition.add(Column::RecordTypeId.eq(record_type_id));
        }

        if let Some(asset_id) = req.asset_id {
            condition = condition.add(Column::AssetId.eq(asset_id));
        }

        let mut stmt = Entity::find().filter(condition);

        stmt = stmt.order_by(Column::Id, sea_orm::Order::Desc);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.as_deref().and_then(parse_order_column) {
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
        "order_id" => Some(Column::OrderId),
        "record_type_id" => Some(Column::RecordTypeId),
        "creation_date" => Some(Column::CreationDate),
        "asset_id" => Some(Column::AssetId),
        "free_amount" => Some(Column::FreeAmount),
        "free_previous_balance" => Some(Column::FreePreviousBalance),
        "free_new_balance" => Some(Column::FreeNewBalance),
        "locked_amount" => Some(Column::LockedAmount),
        "locked_previous_balance" => Some(Column::LockedPreviousBalance),
        "locked_new_balance" => Some(Column::LockedNewBalance),
        _ => None,
    }
}

#[async_trait]
impl Update<LedgerRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: LedgerRequest) -> Result<Model, Response> {
        let req = req.clone();

        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(
                req.id
                    .ok_or(Response::not_found("Missing ledger id".to_string()))?,
            ),
            ..Default::default()
        };

        if let Some(free_amount) = req.free_amount {
            active_model.free_amount = ActiveValue::Set(free_amount);
        }

        if let Some(free_previous_balance) = req.free_previous_balance {
            active_model.free_previous_balance = ActiveValue::Set(free_previous_balance);
        }

        if let Some(free_new_balance) = req.free_new_balance {
            active_model.free_new_balance = ActiveValue::Set(free_new_balance);
        }

        if let Some(locked_amount) = req.locked_amount {
            active_model.locked_amount = ActiveValue::Set(locked_amount);
        }

        if let Some(locked_previous_balance) = req.locked_previous_balance {
            active_model.locked_previous_balance = ActiveValue::Set(locked_previous_balance);
        }

        if let Some(locked_new_balance) = req.locked_new_balance {
            active_model.locked_new_balance = ActiveValue::Set(locked_new_balance);
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
impl Delete<LedgerRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: LedgerRequest) -> Result<u64, Response> {
        let id = req
            .id
            .ok_or(Response::not_found("Missing ledger id".to_string()))?;

        match Entity::delete_by_id(id).exec(&self.data).await {
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
 * MockRepo
 * ========================================================= */

#[async_trait]
impl Insert<LedgerRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: LedgerRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = Model {
            id: req.id.unwrap_or_default(),
            order_id: req.order_id,
            record_type_id: req.record_type_id.unwrap_or_default(),
            creation_date: Local::now().naive_local().into(),
            asset_id: req.asset_id.unwrap_or_default(),
            free_amount: req.free_amount.unwrap_or_default(),
            free_previous_balance: req.free_previous_balance.unwrap_or_default(),
            free_new_balance: req.free_new_balance.unwrap_or_default(),
            locked_amount: req.locked_amount.unwrap_or_default(),
            locked_previous_balance: req.locked_previous_balance.unwrap_or_default(),
            locked_new_balance: req.locked_new_balance.unwrap_or_default(),
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<LedgerRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: LedgerRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| m.id == req.id.unwrap_or_default())
            .cloned())
    }

    async fn select_many(
        &self,
        req: LedgerRequest,
        _query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .filter(|m| req.order_id.map_or(true, |id| m.order_id == Some(id)))
            .cloned()
            .collect())
    }
}

#[async_trait]
impl Update<LedgerRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: LedgerRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        let model = data
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or(Response::not_found("Invalid mock update id".to_string()))?;

        if let Some(v) = req.free_amount {
            model.free_amount = v;
        }

        if let Some(v) = req.free_previous_balance {
            model.free_previous_balance = v;
        }

        if let Some(v) = req.free_new_balance {
            model.free_new_balance = v;
        }

        if let Some(v) = req.locked_amount {
            model.locked_amount = v;
        }

        if let Some(v) = req.locked_previous_balance {
            model.locked_previous_balance = v;
        }

        if let Some(v) = req.locked_new_balance {
            model.locked_new_balance = v;
        }

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<LedgerRequest> for MockRepo<Model> {
    async fn delete(&self, req: LedgerRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or(Response::not_found("Invalid mock delete id".to_string()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
