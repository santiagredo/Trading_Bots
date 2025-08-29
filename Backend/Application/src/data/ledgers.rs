use chrono::Local;
use models::entities::ledgers::{ActiveModel, Column, Entity, Model};
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
};
use tracing::error_span;

use crate::{
    types::Ledgers,
    utils::{handle_db_error, Data, Response},
};

impl Ledgers<Data> {
    pub async fn insert_ledger_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let active_model_ledger = ActiveModel {
            id: ActiveValue::NotSet,
            order_id: ActiveValue::Set(self.model.order_id),
            record_type_id: ActiveValue::Set(self.model.record_type_id.unwrap_or_default()),
            creation_date: ActiveValue::Set(Local::now().naive_local()),
            asset_id: ActiveValue::Set(self.model.asset_id.unwrap_or_default()),
            free_amount: ActiveValue::Set(self.model.free_amount.unwrap_or_default()),
            free_previous_balance: ActiveValue::Set(
                self.model.free_previous_balance.unwrap_or_default(),
            ),
            free_new_balance: ActiveValue::Set(self.model.free_new_balance.unwrap_or_default()),
            locked_amount: ActiveValue::Set(self.model.locked_amount.unwrap_or_default()),
            locked_previous_balance: ActiveValue::Set(
                self.model.locked_previous_balance.unwrap_or_default(),
            ),
            locked_new_balance: ActiveValue::Set(self.model.locked_new_balance.unwrap_or_default()),
        };

        match Entity::insert(active_model_ledger)
            .exec_with_returning(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn select_ledger_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(order_id) = self.model.order_id {
            condition = condition.add(Column::OrderId.eq(order_id))
        }

        if let Some(record_type_id) = self.model.record_type_id {
            condition = condition.add(Column::RecordTypeId.eq(record_type_id))
        }

        if let Some(creation_date) = self.model.creation_date {
            condition = condition.add(Column::CreationDate.eq(creation_date))
        }

        if let Some(asset_id) = self.model.asset_id {
            condition = condition.add(Column::AssetId.eq(asset_id))
        }

        match Entity::find()
            .filter(condition)
            .order_by(Column::CreationDate, sea_orm::Order::Desc)
            .one(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn select_ledgers_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(order_id) = self.model.order_id {
            condition = condition.add(Column::OrderId.eq(order_id))
        }

        if let Some(record_type_id) = self.model.record_type_id {
            condition = condition.add(Column::RecordTypeId.eq(record_type_id))
        }

        if let Some(creation_date) = self.model.creation_date {
            condition = condition.add(Column::CreationDate.eq(creation_date))
        }

        if let Some(asset_id) = self.model.asset_id {
            condition = condition.add(Column::AssetId.eq(asset_id))
        }

        match Entity::find()
            .filter(condition)
            .order_by(Column::CreationDate, sea_orm::Order::Desc)
            .all(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }
}
