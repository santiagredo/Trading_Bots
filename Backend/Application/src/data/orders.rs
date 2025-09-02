use chrono::Local;
use models::entities::orders::{ActiveModel, Column, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};
use tracing::error_span;

use crate::{
    handler::Orders,
    utils::{handle_db_error, Data, Response},
};

impl Orders<Data> {
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
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn select_order_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        match Entity::find()
            .filter(Condition::all().add(Column::Id.eq(self.model.id.unwrap_or_default())))
            .one(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val.unwrap_or_default()),
        }
    }

    // pub async fn select_open_orders(
    //     db: &DatabaseConnection,
    // ) -> Outcome<Vec<Model>, String, String> {
    //     Entity::find()
    //         .filter(Condition::all().add(Column::StatusId.eq(1)))
    //         .all(db)
    //         .await
    //         .map(|val| Outcome::Ok(val))
    //         .map_err(|err| OutcomeError::Error(err.to_string()))?
    // }

    // pub async fn select_completed_orders(
    //     db: &DatabaseConnection,
    // ) -> Outcome<Vec<Model>, String, String> {
    //     let previous_day = Local::now()
    //         .naive_local()
    //         .checked_sub_signed(Duration::days(1))
    //         .unwrap();

    //     Entity::find()
    //         .filter(
    //             Condition::all()
    //                 .add(Column::StatusId.eq(2))
    //                 .add(Column::CreationDate.gte(previous_day)),
    //         )
    //         .order_by_desc(Column::CreationDate)
    //         .all(db)
    //         .await
    //         .map(|val| Outcome::Ok(val))
    //         .map_err(|err| OutcomeError::Error(err.to_string()))?
    // }

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
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }
}
