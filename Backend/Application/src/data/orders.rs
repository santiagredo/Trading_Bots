use chrono::Local;
use function_name::named;
use models::{
    entities::orders::{ActiveModel, Column, Entity, Model},
    structs::ErrorLogRequest,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::{
    handler::{ErrorLogs, Orders},
    log_db_error,
    utils::{Data, Response},
};

impl Orders<Data> {
    #[named]
    pub async fn insert_order_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let now = Local::now().naive_local();

        let active_model_order = ActiveModel {
            id: ActiveValue::NotSet,
            status_id: ActiveValue::Set(self.model.status_id.unwrap_or_default()),
            creation_date: ActiveValue::Set(now.clone().into()),
            update_date: ActiveValue::Set(now.clone()),
            is_sell: ActiveValue::Set(self.model.is_sell.unwrap_or_default()),
            strategy_id: ActiveValue::Set(self.model.strategy_id.unwrap_or_default()),
            base_asset_id: ActiveValue::Set(self.model.base_asset_id.unwrap_or_default()),
            base_asset_amount: ActiveValue::Set(self.model.base_asset_amount.unwrap_or_default()),
            quote_asset_id: ActiveValue::Set(self.model.quote_asset_id.unwrap_or_default()),
            quote_asset_amount: ActiveValue::set(self.model.quote_asset_amount.unwrap_or_default()),
            price_entry: ActiveValue::Set(self.model.price_entry.unwrap_or_default()),
            price_target: ActiveValue::Set(self.model.price_target.unwrap_or_default()),
            price_abort: ActiveValue::Set(self.model.price_abort.unwrap_or_default()),
        };

        match Entity::insert(active_model_order)
            .exec_with_returning(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_order_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        match Entity::find()
            .filter(Condition::all().add(Column::Id.eq(self.model.id.unwrap_or_default())))
            .one(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val.unwrap_or_default()),
        }
    }

    #[named]
    pub async fn select_orders_data(self, db: &DatabaseConnection) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(status_id) = self.model.status_id {
            condition = condition.add(Column::StatusId.eq(status_id));
        }

        if let Some(is_sell) = self.model.is_sell {
            condition = condition.add(Column::IsSell.eq(is_sell));
        }

        if let Some(strategy_id) = self.model.strategy_id {
            condition = condition.add(Column::StrategyId.eq(strategy_id));
        }

        if let Some(base_asset_id) = self.model.base_asset_id {
            condition = condition.add(Column::BaseAssetId.eq(base_asset_id));
        }

        if let Some(quote_asset_id) = self.model.quote_asset_id {
            condition = condition.add(Column::QuoteAssetId.eq(quote_asset_id));
        }

        if let Some(creation_date) = self.model.creation_date {
            condition = condition.add(Column::CreationDate.eq(creation_date));
        }

        if let Some(update_date) = self.model.update_date {
            condition = condition.add(Column::UpdateDate.eq(update_date));
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn update_order_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let active_model_order = ActiveModel {
            id: ActiveValue::Unchanged(self.model.id.unwrap_or_default()),
            status_id: ActiveValue::Set(self.model.status_id.unwrap_or_default()),
            update_date: ActiveValue::Set(Local::now().naive_local().into()),
            base_asset_amount: ActiveValue::Set(self.model.base_asset_amount.unwrap_or_default()),
            quote_asset_amount: ActiveValue::Set(self.model.quote_asset_amount.unwrap_or_default()),
            ..Default::default()
        };

        match active_model_order.update(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }
}
