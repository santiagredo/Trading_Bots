use chrono::Local;
use models::entities::ledgers::{ActiveModel, Column, Entity, Model};
use sea_orm::{
    ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
};

use crate::{
    types::Ledgers,
    utils::{Data, Outcome, OutcomeError},
};

impl Ledgers<Data> {
    pub async fn insert_ledger(
        db: &DatabaseConnection,
        ledger_type: Self,
    ) -> Outcome<Model, String, String> {
        let active_model_ledger = ActiveModel {
            id: ActiveValue::NotSet,
            order_id: ActiveValue::Set(ledger_type.model.order_id),
            record_type_id: ActiveValue::Set(ledger_type.model.record_type_id),
            creation_date: ActiveValue::Set(Local::now().naive_local()),
            base_asset_id: ActiveValue::Set(ledger_type.model.base_asset_id),
            base_asset_amount: ActiveValue::Set(ledger_type.model.base_asset_amount),
            base_asset_previous_balance: ActiveValue::Set(
                ledger_type.model.base_asset_previous_balance,
            ),
            base_asset_new_balance: ActiveValue::Set(
                ledger_type.model.base_asset_new_balance,
            ),
            quote_asset_id: ActiveValue::Set(ledger_type.model.quote_asset_id),
            quote_asset_amount: ActiveValue::Set(ledger_type.model.quote_asset_amount),
            quote_asset_previous_balance: ActiveValue::Set(
                ledger_type.model.quote_asset_previous_balance,
            ),
            quote_asset_new_balance: ActiveValue::Set(ledger_type.model.quote_asset_new_balance),
        };

        Entity::insert(active_model_ledger)
            .exec_with_returning(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }

    pub async fn select_ledger(db: &DatabaseConnection, id: i32) -> Outcome<Model, String, String> {
        Entity::find()
            .filter(Condition::all().add(Column::Id.eq(id)))
            .one(db)
            .await
            .map_err(|err| OutcomeError::Error(err.to_string()))?
            .ok_or_else(|| OutcomeError::Failure("Ledger not found".to_string()))
    }

    pub async fn select_ledgers(db: &DatabaseConnection) -> Outcome<Vec<Model>, String, String> {
        let ledgers = Entity::find()
            .order_by(Column::CreationDate, sea_orm::Order::Desc)
            .all(db)
            .await
            .map_err(|err| OutcomeError::Error(err.to_string()))?;

        Outcome::Ok(ledgers)
    }
}
