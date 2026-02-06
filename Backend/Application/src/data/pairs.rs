use crate::handler::ErrorLogs;
use crate::log_trait_db_error;
use crate::utils::{handle_db_error, Response};
use crate::utils::{DbRepo, Delete, Insert, MockRepo, Select, Update};
use chrono::Local;
use function_name::named;
use migration::async_trait::async_trait;
use models::structs::PairRequest;
use models::{
    entities::pairs::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::QueryOptions,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, EntityTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

#[async_trait]
impl Insert<PairRequest, Model> for DbRepo {
    #[named]
    async fn insert(&self, req: PairRequest) -> Result<Model, Response> {
        let req_for_log = req.clone();
        let now = Local::now().naive_local();

        let active_model = ActiveModel {
            id: ActiveValue::NotSet,
            base_asset_id: ActiveValue::Set(req.base_asset_id.unwrap_or_default()),
            quote_asset_id: ActiveValue::Set(req.quote_asset_id.unwrap_or_default()),
            symbol: ActiveValue::Set(req.symbol.unwrap_or_default()),
            update_date: ActiveValue::Set(now.into()),
            ..Default::default()
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
impl Select<PairRequest, Model> for DbRepo {
    #[named]
    async fn select(&self, req: PairRequest) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(base_asset_id) = req.base_asset_id {
            condition = condition.add(Column::BaseAssetId.eq(base_asset_id));
        }

        if let Some(quote_asset_id) = req.quote_asset_id {
            condition = condition.add(Column::QuoteAssetId.eq(quote_asset_id));
        }

        if let Some(symbol) = req.symbol.as_deref().filter(|s| !s.is_empty()) {
            condition = condition.add(Column::Symbol.eq(symbol));
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
        req: PairRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = req.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(base_asset_id) = req.base_asset_id {
            condition = condition.add(Column::BaseAssetId.eq(base_asset_id));
        }

        if let Some(quote_asset_id) = req.quote_asset_id {
            condition = condition.add(Column::QuoteAssetId.eq(quote_asset_id));
        }

        if let Some(symbol) = req.symbol.as_deref().filter(|s| !s.is_empty()) {
            condition = condition.add(Column::Symbol.eq(symbol));
        }

        let mut stmt = Entity::find().filter(condition);

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
        "base_asset_id" => Some(Column::BaseAssetId),
        "quote_asset_id" => Some(Column::QuoteAssetId),
        "symbol" => Some(Column::Symbol),
        "update_date" => Some(Column::UpdateDate),
        "all_time_high_price" => Some(Column::AllTimeHighPrice),
        "percent_from_all_time_high" => Some(Column::PercentFromAllTimeHigh),
        "day_price_percent_change" => Some(Column::DayPricePercentChange),
        "week_price_percent_change" => Some(Column::WeekPricePercentChange),
        "month_price_percent_change" => Some(Column::MonthPricePercentChange),
        "year_price_percent_change" => Some(Column::YearPricePercentChange),
        _ => None,
    }
}

#[async_trait]
impl Update<PairRequest, Model> for DbRepo {
    #[named]
    async fn update(&self, req: PairRequest) -> Result<Model, Response> {
        let mut active_model = ActiveModel {
            id: ActiveValue::Unchanged(
                req.id
                    .ok_or(Response::not_found("Invalid pair id".to_string()))?,
            ),
            update_date: ActiveValue::Set(Local::now().naive_local().into()),
            ..Default::default()
        };

        // ============================
        // Identity
        // ============================

        if let Some(val) = req.base_asset_id {
            active_model.base_asset_id = ActiveValue::Set(val);
        }

        if let Some(val) = req.quote_asset_id {
            active_model.quote_asset_id = ActiveValue::Set(val);
        }

        if let Some(val) = req.symbol.as_ref() {
            active_model.symbol = ActiveValue::Set(val.clone());
        }

        // ============================
        // Prices / metrics
        // ============================

        if let Some(val) = req.last_price {
            active_model.last_price = ActiveValue::Set(val);
        }

        if let Some(val) = req.all_time_high_price {
            active_model.all_time_high_price = ActiveValue::Set(val);
        }

        if let Some(val) = req.all_time_high_date {
            active_model.all_time_high_date = ActiveValue::Set(val);
        }

        if let Some(val) = req.percent_from_all_time_high {
            active_model.percent_from_all_time_high = ActiveValue::Set(val);
        }

        if let Some(val) = req.fifteen_minutes_price_percent_change {
            active_model.fifteen_minutes_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = req.thirty_minutes_price_percent_change {
            active_model.thirty_minutes_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = req.hour_price_percent_change {
            active_model.hour_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = req.six_hours_price_percent_change {
            active_model.six_hours_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = req.twelve_hours_price_percent_change {
            active_model.twelve_hours_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = req.day_price_percent_change {
            active_model.day_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = req.week_price_percent_change {
            active_model.week_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = req.month_price_percent_change {
            active_model.month_price_percent_change = ActiveValue::Set(val);
        }

        if let Some(val) = req.year_price_percent_change {
            active_model.year_price_percent_change = ActiveValue::Set(val);
        }

        // ============================
        // Price filters
        // ============================

        if let Some(val) = req.price_filter_min_price {
            active_model.price_filter_min_price = ActiveValue::Set(val);
        }

        if let Some(val) = req.price_filter_max_price {
            active_model.price_filter_max_price = ActiveValue::Set(val);
        }

        if let Some(val) = req.price_filter_tick_size {
            active_model.price_filter_tick_size = ActiveValue::Set(val);
        }

        // ============================
        // Lot size
        // ============================

        if let Some(val) = req.lot_size_min_qty {
            active_model.lot_size_min_qty = ActiveValue::Set(val);
        }

        if let Some(val) = req.lot_size_max_qty {
            active_model.lot_size_max_qty = ActiveValue::Set(val);
        }

        if let Some(val) = req.lot_size_step_size {
            active_model.lot_size_step_size = ActiveValue::Set(val);
        }

        // ============================
        // Market lot size
        // ============================

        if let Some(val) = req.market_lot_size_min_qty {
            active_model.market_lot_size_min_qty = ActiveValue::Set(val);
        }

        if let Some(val) = req.market_lot_size_max_qty {
            active_model.market_lot_size_max_qty = ActiveValue::Set(val);
        }

        if let Some(val) = req.market_lot_size_step_size {
            active_model.market_lot_size_step_size = ActiveValue::Set(val);
        }

        // ============================
        // Trailing delta
        // ============================

        if let Some(val) = req.trailing_delta_min_trailing_above_delta {
            active_model.trailing_delta_min_trailing_above_delta = ActiveValue::Set(val);
        }

        if let Some(val) = req.trailing_delta_max_trailing_above_delta {
            active_model.trailing_delta_max_trailing_above_delta = ActiveValue::Set(val);
        }

        if let Some(val) = req.trailing_delta_min_trailing_below_delta {
            active_model.trailing_delta_min_trailing_below_delta = ActiveValue::Set(val);
        }

        if let Some(val) = req.trailing_delta_max_trailing_below_delta {
            active_model.trailing_delta_max_trailing_below_delta = ActiveValue::Set(val);
        }

        // ============================
        // Percent price by side
        // ============================

        if let Some(val) = req.percent_price_by_side_bid_multiplier_up {
            active_model.percent_price_by_side_bid_multiplier_up = ActiveValue::Set(val);
        }

        if let Some(val) = req.percent_price_by_side_bid_multiplier_down {
            active_model.percent_price_by_side_bid_multiplier_down = ActiveValue::Set(val);
        }

        if let Some(val) = req.percent_price_by_side_ask_multiplier_up {
            active_model.percent_price_by_side_ask_multiplier_up = ActiveValue::Set(val);
        }

        if let Some(val) = req.percent_price_by_side_ask_multiplier_down {
            active_model.percent_price_by_side_ask_multiplier_down = ActiveValue::Set(val);
        }

        if let Some(val) = req.percent_price_by_side_avg_price_mins {
            active_model.percent_price_by_side_avg_price_mins = ActiveValue::Set(val);
        }

        // ============================
        // Notional
        // ============================

        if let Some(val) = req.notional_min_notional {
            active_model.notional_min_notional = ActiveValue::Set(val);
        }

        if let Some(val) = req.notional_apply_min_to_market {
            active_model.notional_apply_min_to_market = ActiveValue::Set(val);
        }

        if let Some(val) = req.notional_max_notional {
            active_model.notional_max_notional = ActiveValue::Set(val);
        }

        if let Some(val) = req.notional_apply_max_to_market {
            active_model.notional_apply_max_to_market = ActiveValue::Set(val);
        }

        if let Some(val) = req.notional_avg_price_mins {
            active_model.notional_avg_price_mins = ActiveValue::Set(val);
        }

        // ============================
        // Limits
        // ============================

        if let Some(val) = req.max_num_orders {
            active_model.max_num_orders = ActiveValue::Set(val);
        }

        if let Some(val) = req.max_num_algo_orders {
            active_model.max_num_algo_orders = ActiveValue::Set(val);
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
impl Delete<PairRequest> for DbRepo {
    #[named]
    async fn delete(&self, req: PairRequest) -> Result<u64, Response> {
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
impl Insert<PairRequest, Model> for MockRepo<Model> {
    async fn insert(&self, req: PairRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let model = Model {
            id: req.id.unwrap_or_else(|| (data.len() as i32) + 1),
            base_asset_id: req.base_asset_id.unwrap_or_default(),
            quote_asset_id: req.quote_asset_id.unwrap_or_default(),
            symbol: req.symbol.unwrap_or_default(),
            update_date: Local::now().naive_local().into(),
            last_price: req.last_price.unwrap_or_default(),
            all_time_high_price: req.all_time_high_price.unwrap_or_default(),
            all_time_high_date: req.all_time_high_date.unwrap_or_default(),
            percent_from_all_time_high: req.percent_from_all_time_high.unwrap_or_default(),
            day_price_percent_change: req.day_price_percent_change.unwrap_or_default(),
            week_price_percent_change: req.week_price_percent_change.unwrap_or_default(),
            month_price_percent_change: req.month_price_percent_change.unwrap_or_default(),
            year_price_percent_change: req.year_price_percent_change.unwrap_or_default(),
            ..Default::default()
        };

        data.push(model.clone());
        Ok(model)
    }
}

#[async_trait]
impl Select<PairRequest, Model> for MockRepo<Model> {
    async fn select(&self, req: PairRequest) -> Result<Option<Model>, Response> {
        let data = self.data.lock().unwrap();

        Ok(data
            .iter()
            .find(|m| {
                if let Some(id) = req.id {
                    if m.id != id {
                        return false;
                    }
                }

                if let Some(base_asset_id) = req.base_asset_id {
                    if m.base_asset_id != base_asset_id {
                        return false;
                    }
                }

                if let Some(quote_asset_id) = req.quote_asset_id {
                    if m.quote_asset_id != quote_asset_id {
                        return false;
                    }
                }

                if let Some(symbol) = req.symbol.as_deref() {
                    if m.symbol != symbol {
                        return false;
                    }
                }

                true
            })
            .cloned())
    }

    async fn select_many(
        &self,
        req: PairRequest,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let data = self.data.lock().unwrap();

        let mut result: Vec<Model> = data
            .iter()
            .filter(|m| {
                if let Some(id) = req.id {
                    if m.id != id {
                        return false;
                    }
                }

                if let Some(base_asset_id) = req.base_asset_id {
                    if m.base_asset_id != base_asset_id {
                        return false;
                    }
                }

                if let Some(quote_asset_id) = req.quote_asset_id {
                    if m.quote_asset_id != quote_asset_id {
                        return false;
                    }
                }

                if let Some(symbol) = req.symbol.as_deref() {
                    if m.symbol != symbol {
                        return false;
                    }
                }

                true
            })
            .cloned()
            .collect();

        if let Some(q) = query {
            if let Some(offset) = q.offset {
                result = result.into_iter().skip(offset as usize).collect();
            }

            if let Some(limit) = q.limit {
                result.truncate(limit as usize);
            }
        }

        Ok(result)
    }
}

#[async_trait]
impl Update<PairRequest, Model> for MockRepo<Model> {
    async fn update(&self, req: PairRequest) -> Result<Model, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or_else(|| Response::not_found("Invalid mock update id".to_string()))?;

        let model = data
            .iter_mut()
            .find(|m| m.id == id)
            .ok_or_else(|| Response::not_found("Invalid mock update id".to_string()))?;

        if let Some(val) = req.base_asset_id {
            model.base_asset_id = val;
        }

        if let Some(val) = req.quote_asset_id {
            model.quote_asset_id = val;
        }

        if let Some(val) = req.symbol {
            model.symbol = val;
        }

        if let Some(val) = req.last_price {
            model.last_price = val;
        }

        if let Some(val) = req.all_time_high_price {
            model.all_time_high_price = val;
        }

        if let Some(val) = req.all_time_high_date {
            model.all_time_high_date = val;
        }

        if let Some(val) = req.percent_from_all_time_high {
            model.percent_from_all_time_high = val;
        }

        if let Some(val) = req.day_price_percent_change {
            model.day_price_percent_change = val;
        }

        if let Some(val) = req.week_price_percent_change {
            model.week_price_percent_change = val;
        }

        if let Some(val) = req.month_price_percent_change {
            model.month_price_percent_change = val;
        }

        if let Some(val) = req.year_price_percent_change {
            model.year_price_percent_change = val;
        }

        model.update_date = Local::now().naive_local().into();

        Ok(model.clone())
    }
}

#[async_trait]
impl Delete<PairRequest> for MockRepo<Model> {
    async fn delete(&self, req: PairRequest) -> Result<u64, Response> {
        let mut data = self.data.lock().unwrap();

        let id = req
            .id
            .ok_or_else(|| Response::not_found("Invalid mock delete id".to_string()))?;

        let before = data.len();
        data.retain(|m| m.id != id);

        Ok((before - data.len()) as u64)
    }
}
