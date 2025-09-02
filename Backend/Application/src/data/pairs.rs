use chrono::Local;
use models::entities::pairs::{ActiveModel, Column, Entity, Model};
use sea_orm::{ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error_span;

use crate::{
    handler::Pairs,
    utils::{handle_db_error, Data, Response},
};

impl Pairs<Data> {
    pub async fn insert_pair_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let active_model_pair = ActiveModel {
            id: ActiveValue::NotSet,
            base_asset_id: ActiveValue::Set(self.model.base_asset_id.unwrap_or_default()),
            quote_asset_id: ActiveValue::Set(self.model.quote_asset_id.unwrap_or_default()),
            symbol: ActiveValue::Set(self.model.symbol.unwrap_or_default()),
            ..Default::default()
        };

        match Entity::insert(active_model_pair)
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

    pub async fn select_pair_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if self.model.id.is_some_and(|id| id > 0) {
            condition = condition.add(Column::Id.eq(self.model.id.unwrap_or_default()));
        }

        if self.model.base_asset_id.is_some_and(|id| id > 0) {
            condition =
                condition.add(Column::BaseAssetId.eq(self.model.base_asset_id.unwrap_or_default()));
        }

        if self.model.quote_asset_id.is_some_and(|id| id > 0) {
            condition = condition
                .add(Column::QuoteAssetId.eq(self.model.quote_asset_id.unwrap_or_default()));
        }

        if self
            .model
            .symbol
            .as_ref()
            .is_some_and(|symbol| !symbol.is_empty())
        {
            condition = condition.add(Column::Symbol.eq(self.model.symbol.unwrap_or_default()));
        }

        match Entity::find().filter(condition).one(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }

    pub async fn select_pairs_data(self, db: &DatabaseConnection) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if self.model.id.is_some_and(|id| id > 0) {
            condition = condition.add(Column::Id.eq(self.model.id.unwrap_or_default()));
        }

        if self.model.base_asset_id.is_some_and(|id| id > 0) {
            condition =
                condition.add(Column::BaseAssetId.eq(self.model.base_asset_id.unwrap_or_default()));
        }

        if self.model.quote_asset_id.is_some_and(|id| id > 0) {
            condition = condition
                .add(Column::QuoteAssetId.eq(self.model.quote_asset_id.unwrap_or_default()));
        }

        if self
            .model
            .symbol
            .as_ref()
            .is_some_and(|symbol| !symbol.is_empty())
        {
            condition = condition.add(Column::Symbol.eq(self.model.symbol.unwrap_or_default()));
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => Ok(val),
        }
    }

    // pub async fn select_all_pairs(db: &DatabaseConnection) -> Outcome<Vec<Model>, String, String> {
    //     Entity::find()
    //         .all(db)
    //         .await
    //         .map(|val| Outcome::Ok(val))
    //         .map_err(|err| OutcomeError::Error(err.to_string()))?
    // }

    // pub async fn select_pairs_by_ids(
    //     db: &DatabaseConnection,
    //     ids: Vec<i32>,
    // ) -> Outcome<Vec<Model>, String, String> {
    //     Entity::find()
    //         .filter(Column::Id.is_in(ids))
    //         .all(db)
    //         .await
    //         .map(|val| Outcome::Ok(val))
    //         .map_err(|err| OutcomeError::Error(err.to_string()))?
    // }

    pub async fn update_pair_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let now = Local::now().naive_local();

        let mut active_model_pair = ActiveModel {
            id: ActiveValue::Unchanged(self.model.id.unwrap_or_default()),
            update_date: ActiveValue::Set(now.into()),
            ..Default::default()
        };

        if let Some(val) = self.model.base_asset_id {
            active_model_pair.base_asset_id = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.quote_asset_id {
            active_model_pair.quote_asset_id = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.symbol.clone() {
            active_model_pair.symbol = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.all_time_high_price {
            active_model_pair.all_time_high_price = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.all_time_high_date {
            active_model_pair.all_time_high_date = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.percent_from_all_time_high {
            active_model_pair.percent_from_all_time_high = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.fifteen_minutes_price_percent_change {
            active_model_pair.fifteen_minutes_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.thirty_minutes_price_percent_change {
            active_model_pair.thirty_minutes_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.hour_price_percent_change {
            active_model_pair.hour_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.six_hours_price_percent_change {
            active_model_pair.six_hours_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.twelve_hours_price_percent_change {
            active_model_pair.twelve_hours_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.day_price_percent_change {
            active_model_pair.day_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.week_price_percent_change {
            active_model_pair.week_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.month_price_percent_change {
            active_model_pair.month_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.year_price_percent_change {
            active_model_pair.year_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.price_filter_min_price {
            active_model_pair.price_filter_min_price = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.price_filter_max_price {
            active_model_pair.price_filter_max_price = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.price_filter_tick_size {
            active_model_pair.price_filter_tick_size = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.lot_size_min_qty {
            active_model_pair.lot_size_min_qty = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.lot_size_max_qty {
            active_model_pair.lot_size_max_qty = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.lot_size_step_size {
            active_model_pair.lot_size_step_size = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.iceberg_parts_limit {
            active_model_pair.iceberg_parts_limit = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.market_lot_size_min_qty {
            active_model_pair.market_lot_size_min_qty = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.market_lot_size_max_qty {
            active_model_pair.market_lot_size_max_qty = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.market_lot_size_step_size {
            active_model_pair.market_lot_size_step_size = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.trailing_delta_min_trailing_above_delta {
            active_model_pair.trailing_delta_min_trailing_above_delta = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.trailing_delta_max_trailing_above_delta {
            active_model_pair.trailing_delta_max_trailing_above_delta = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.trailing_delta_min_trailing_below_delta {
            active_model_pair.trailing_delta_min_trailing_below_delta = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.trailing_delta_max_trailing_below_delta {
            active_model_pair.trailing_delta_max_trailing_below_delta = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.percent_price_by_side_bid_multiplier_up {
            active_model_pair.percent_price_by_side_bid_multiplier_up = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.percent_price_by_side_bid_multiplier_down {
            active_model_pair.percent_price_by_side_bid_multiplier_down = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.percent_price_by_side_ask_multiplier_up {
            active_model_pair.percent_price_by_side_ask_multiplier_up = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.percent_price_by_side_ask_multiplier_down {
            active_model_pair.percent_price_by_side_ask_multiplier_down = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.percent_price_by_side_avg_price_mins {
            active_model_pair.percent_price_by_side_avg_price_mins = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.notional_min_notional {
            active_model_pair.notional_min_notional = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.notional_apply_min_to_market {
            active_model_pair.notional_apply_min_to_market = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.notional_max_notional {
            active_model_pair.notional_max_notional = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.notional_apply_max_to_market {
            active_model_pair.notional_apply_max_to_market = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.notional_avg_price_mins {
            active_model_pair.notional_avg_price_mins = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.max_num_orders {
            active_model_pair.max_num_orders = ActiveValue::Set(val);
        }

        if let Some(val) = self.model.max_num_algo_orders {
            active_model_pair.max_num_algo_orders = ActiveValue::Set(val);
        }

        match Entity::update(active_model_pair).exec(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);
                Err(handle_db_error(&err))
            }
            Ok(val) => Ok(val),
        }
    }
}
