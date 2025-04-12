use chrono::Local;
use models::entities::pair_assets::{ActiveModel, Column, Entity, Model};
use sea_orm::{ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    types::PairAssets,
    utils::{Data, Outcome, OutcomeError},
};

impl PairAssets<Data> {
    pub async fn insert_pair_asset(
        db: &DatabaseConnection,
        pair_asset_type: Self,
    ) -> Outcome<Model, String, String> {
        let active_model_pair_asset = ActiveModel {
            id: ActiveValue::NotSet,
            base_asset_id: ActiveValue::Set(pair_asset_type.model.base_asset_id),
            quote_asset_id: ActiveValue::Set(pair_asset_type.model.quote_asset_id),
            symbol: ActiveValue::Set(pair_asset_type.model.symbol),
            ..Default::default()
        };

        Entity::insert(active_model_pair_asset)
            .exec_with_returning(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }

    pub async fn select_pair_asset(
        db: &DatabaseConnection,
        pair_asset_type: Self,
    ) -> Outcome<Model, String, String> {
        let mut condition = Condition::all();

        if pair_asset_type.model.id > 0 {
            condition = condition.add(Column::Id.eq(pair_asset_type.model.id));
        }

        if pair_asset_type.model.base_asset_id > 0 {
            condition = condition.add(Column::BaseAssetId.eq(pair_asset_type.model.base_asset_id));
        }

        if pair_asset_type.model.quote_asset_id > 0 {
            condition =
                condition.add(Column::QuoteAssetId.eq(pair_asset_type.model.quote_asset_id));
        }

        if !pair_asset_type.model.symbol.is_empty() {
            condition = condition.add(Column::Symbol.eq(pair_asset_type.model.symbol));
        }

        Entity::find()
            .filter(condition)
            .one(db)
            .await
            .map_err(|err| OutcomeError::Error(err.to_string()))?
            .ok_or_else(|| OutcomeError::Failure("Pair asset not found".to_string()))
    }

    pub async fn select_all_pair_assets(
        db: &DatabaseConnection,
    ) -> Outcome<Vec<Model>, String, String> {
        Entity::find()
            .all(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }

    pub async fn update_pair_asset(
        db: &DatabaseConnection,
        pair_asset_type: Self,
    ) -> Outcome<Model, String, String> {
        let now = Local::now().naive_local();

        let active_model_pair_asset = ActiveModel {
            id: ActiveValue::Unchanged(pair_asset_type.model.id),
            base_asset_id: ActiveValue::Set(pair_asset_type.model.base_asset_id),
            quote_asset_id: ActiveValue::Set(pair_asset_type.model.quote_asset_id),
            symbol: ActiveValue::Set(pair_asset_type.model.symbol),
            update_date: ActiveValue::Set(now.into()),

            // ATH-related fields
            all_time_high_price: ActiveValue::Set(pair_asset_type.model.all_time_high_price),
            all_time_high_date: ActiveValue::Set(pair_asset_type.model.all_time_high_date),
            percent_from_all_time_high: ActiveValue::Set(
                pair_asset_type.model.percent_from_all_time_high,
            ),

            // Price percent changes
            fifteen_minutes_price_percent_change: ActiveValue::Set(
                pair_asset_type.model.fifteen_minutes_price_percent_change,
            ),
            thirty_minutes_price_percent_change: ActiveValue::Set(
                pair_asset_type.model.thirty_minutes_price_percent_change,
            ),
            hour_price_percent_change: ActiveValue::Set(
                pair_asset_type.model.hour_price_percent_change,
            ),
            six_hours_price_percent_change: ActiveValue::Set(
                pair_asset_type.model.six_hours_price_percent_change,
            ),
            twelve_hours_price_percent_change: ActiveValue::Set(
                pair_asset_type.model.twelve_hours_price_percent_change,
            ),
            day_price_percent_change: ActiveValue::Set(
                pair_asset_type.model.day_price_percent_change,
            ),
            week_price_percent_change: ActiveValue::Set(
                pair_asset_type.model.week_price_percent_change,
            ),
            month_price_percent_change: ActiveValue::Set(
                pair_asset_type.model.month_price_percent_change,
            ),
            year_price_percent_change: ActiveValue::Set(
                pair_asset_type.model.year_price_percent_change,
            ),

            // Price filters (Exchange Rules)
            price_filter_min_price: ActiveValue::Set(pair_asset_type.model.price_filter_min_price),
            price_filter_max_price: ActiveValue::Set(pair_asset_type.model.price_filter_max_price),
            price_filter_tick_size: ActiveValue::Set(pair_asset_type.model.price_filter_tick_size),

            // Lot size filters (Exchange Rules)
            lot_size_min_qty: ActiveValue::Set(pair_asset_type.model.lot_size_min_qty),
            lot_size_max_qty: ActiveValue::Set(pair_asset_type.model.lot_size_max_qty),
            lot_size_step_size: ActiveValue::Set(pair_asset_type.model.lot_size_step_size),

            iceberg_parts_limit: ActiveValue::Set(pair_asset_type.model.iceberg_parts_limit),

            // Market Lot Size (Exchange Rules)
            market_lot_size_min_qty: ActiveValue::Set(
                pair_asset_type.model.market_lot_size_min_qty,
            ),
            market_lot_size_max_qty: ActiveValue::Set(
                pair_asset_type.model.market_lot_size_max_qty,
            ),
            market_lot_size_step_size: ActiveValue::Set(
                pair_asset_type.model.market_lot_size_step_size,
            ),

            // Trailing Delta values
            trailing_delta_min_trailing_above_delta: ActiveValue::Set(
                pair_asset_type
                    .model
                    .trailing_delta_min_trailing_above_delta,
            ),
            trailing_delta_max_trailing_above_delta: ActiveValue::Set(
                pair_asset_type
                    .model
                    .trailing_delta_max_trailing_above_delta,
            ),
            trailing_delta_min_trailing_below_delta: ActiveValue::Set(
                pair_asset_type
                    .model
                    .trailing_delta_min_trailing_below_delta,
            ),
            trailing_delta_max_trailing_below_delta: ActiveValue::Set(
                pair_asset_type
                    .model
                    .trailing_delta_max_trailing_below_delta,
            ),

            // Percent price by side multipliers
            percent_price_by_side_bid_multiplier_up: ActiveValue::Set(
                pair_asset_type
                    .model
                    .percent_price_by_side_bid_multiplier_up,
            ),
            percent_price_by_side_bid_multiplier_down: ActiveValue::Set(
                pair_asset_type
                    .model
                    .percent_price_by_side_bid_multiplier_down,
            ),
            percent_price_by_side_ask_multiplier_up: ActiveValue::Set(
                pair_asset_type
                    .model
                    .percent_price_by_side_ask_multiplier_up,
            ),
            percent_price_by_side_ask_multiplier_down: ActiveValue::Set(
                pair_asset_type
                    .model
                    .percent_price_by_side_ask_multiplier_down,
            ),
            percent_price_by_side_avg_price_mins: ActiveValue::Set(
                pair_asset_type.model.percent_price_by_side_avg_price_mins,
            ),

            // Notional values
            notional_min_notional: ActiveValue::Set(pair_asset_type.model.notional_min_notional),
            notional_apply_min_to_market: ActiveValue::Set(
                pair_asset_type.model.notional_apply_min_to_market,
            ),
            notional_max_notional: ActiveValue::Set(pair_asset_type.model.notional_max_notional),
            notional_apply_max_to_market: ActiveValue::Set(
                pair_asset_type.model.notional_apply_max_to_market,
            ),
            notional_avg_price_mins: ActiveValue::Set(
                pair_asset_type.model.notional_avg_price_mins,
            ),

            // Order limits
            max_num_orders: ActiveValue::Set(pair_asset_type.model.max_num_orders),
            max_num_algo_orders: ActiveValue::Set(pair_asset_type.model.max_num_algo_orders),
        };

        Entity::update(active_model_pair_asset)
            .exec(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }
}
