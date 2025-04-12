use chrono::{Duration, Local};
use models::entities::orders::{ActiveModel, Column, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder,
};

use crate::{
    types::Orders,
    utils::{Data, Outcome, OutcomeError},
};

impl Orders<Data> {
    pub async fn insert_order(
        db: &DatabaseConnection,
        orders_type: Self,
    ) -> Outcome<Model, String, String> {
        let now = Local::now().naive_local();

        let active_model_order = ActiveModel {
            id: ActiveValue::NotSet,
            status_id: ActiveValue::Set(orders_type.model.status_id),
            creation_date: ActiveValue::Set(now.clone().into()),
            update_date: ActiveValue::Set(now.clone()),
            is_sell: ActiveValue::Set(orders_type.model.is_sell),
            strategy_id: ActiveValue::Set(orders_type.model.strategy_id),
            base_asset_id: ActiveValue::Set(orders_type.model.base_asset_id),
            base_asset_amount: ActiveValue::Set(orders_type.model.base_asset_amount),
            quote_asset_id: ActiveValue::Set(orders_type.model.quote_asset_id),
            quote_asset_amount: ActiveValue::set(orders_type.model.quote_asset_amount),
            price_entry: ActiveValue::Set(orders_type.model.price_entry),
            price_target: ActiveValue::Set(orders_type.model.price_target),
            price_abort: ActiveValue::Set(orders_type.model.price_abort),
        };

        Entity::insert(active_model_order)
            .exec_with_returning(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }

    pub async fn select_order(db: &DatabaseConnection, id: i32) -> Outcome<Model, String, String> {
        Entity::find()
            .filter(Condition::all().add(Column::Id.eq(id)))
            .one(db)
            .await
            .map_err(|err| OutcomeError::Error(err.to_string()))?
            .ok_or_else(|| OutcomeError::Failure("Order not found".to_string()))
    }

    pub async fn select_open_orders(
        db: &DatabaseConnection,
    ) -> Outcome<Vec<Model>, String, String> {
        Entity::find()
            .filter(Condition::all().add(Column::StatusId.eq(1)))
            .all(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }

    pub async fn select_completed_orders(
        db: &DatabaseConnection,
    ) -> Outcome<Vec<Model>, String, String> {
        let previous_day = Local::now()
            .naive_local()
            .checked_sub_signed(Duration::days(1))
            .unwrap();

        Entity::find()
            .filter(
                Condition::all()
                    .add(Column::StatusId.eq(2))
                    .add(Column::CreationDate.gte(previous_day)),
            )
            .order_by_desc(Column::CreationDate)
            .all(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }

    pub async fn update_order(
        db: &DatabaseConnection,
        orders_type: Self,
    ) -> Outcome<Model, String, String> {
        let active_model_order = ActiveModel {
            id: ActiveValue::Unchanged(orders_type.model.id),
            status_id: ActiveValue::Set(orders_type.model.status_id),
            update_date: ActiveValue::Set(Local::now().naive_local().into()),
            base_asset_amount: ActiveValue::Set(orders_type.model.base_asset_amount),
            quote_asset_amount: ActiveValue::Set(orders_type.model.quote_asset_amount),
            ..Default::default()
        };

        active_model_order
            .update(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }
}
