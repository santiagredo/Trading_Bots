use chrono::Local;
use function_name::named;
use models::{
    entities::actions::{ActiveModel, Column, Entity, Model},
    enums::OrderDirection,
    structs::{ErrorLogRequest, QueryOptions},
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect,
};

use crate::{
    handler::{Actions, ErrorLogs},
    log_db_error,
    utils::{Data, Response},
};

impl Actions<Data> {
    #[named]
    pub async fn insert_action_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let now = Local::now();

        let active_model_action = ActiveModel {
            id: ActiveValue::NotSet,
            strategy_id: ActiveValue::Set(self.model.strategy_id.unwrap_or_default()),
            is_active: ActiveValue::Set(self.model.is_active.unwrap_or_default()),
            is_sell: ActiveValue::Set(self.model.is_sell.unwrap_or_default()),
            is_quote_asset: ActiveValue::Set(self.model.is_quote_asset.unwrap_or_default()),
            is_percentage: ActiveValue::Set(self.model.is_percentage.unwrap_or_default()),
            value: ActiveValue::Set(self.model.value.unwrap_or_default()),
            pair_id: ActiveValue::Set(self.model.pair_id.unwrap_or_default()),
            last_update: ActiveValue::Set(now.naive_local().into()),
        };

        match Entity::insert(active_model_action)
            .exec_with_returning(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_action_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Option<Model>, Response> {
        let mut condition = Condition::all();

        if self.model.id.is_some() {
            condition = condition.add(Column::Id.eq(self.model.id.unwrap_or_default()))
        }

        if self.model.strategy_id.is_some() {
            condition =
                condition.add(Column::StrategyId.eq(self.model.strategy_id.unwrap_or_default()))
        }

        if self.model.pair_id.is_some() {
            condition = condition.add(Column::PairId.eq(self.model.pair_id.unwrap_or_default()))
        }

        match Entity::find().filter(condition).one(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_actions_data(
        self,
        db: &DatabaseConnection,
        query: Option<QueryOptions>,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(strategy_id) = self.model.strategy_id {
            condition = condition.add(Column::StrategyId.eq(strategy_id));
        }

        if let Some(is_active) = self.model.is_active {
            condition = condition.add(Column::IsActive.eq(is_active));
        }

        if let Some(pair_id) = self.model.pair_id {
            condition = condition.add(Column::PairId.eq(pair_id));
        }

        let mut stmt = Entity::find().filter(condition);

        if let Some(q) = query {
            if let Some(limit) = q.limit {
                stmt = stmt.limit(limit);
            }

            if let Some(offset) = q.offset {
                stmt = stmt.offset(offset);
            }

            if let Some(order_by) = q.order_by.and_then(|c| Self::parse_order_column(&c)) {
                let direction = match q.order_direction.unwrap_or(OrderDirection::Asc) {
                    OrderDirection::Asc => sea_orm::Order::Asc,
                    OrderDirection::Desc => sea_orm::Order::Desc,
                };

                stmt = stmt.order_by(order_by, direction);
            }
        }

        match stmt.all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    fn parse_order_column(value: &str) -> Option<Column> {
        match value {
            "id" => Some(Column::Id),
            "strategy_id" => Some(Column::StrategyId),
            "is_active" => Some(Column::IsActive),
            "is_sell" => Some(Column::IsSell),
            "is_quote_asset" => Some(Column::IsQuoteAsset),
            "is_percentage" => Some(Column::IsPercentage),
            "value" => Some(Column::Value),
            "pair_id" => Some(Column::PairId),
            "last_update" => Some(Column::LastUpdate),
            _ => None,
        }
    }

    #[named]
    pub async fn update_action_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let mut active_model_action = ActiveModel {
            id: ActiveValue::Unchanged(self.model.id.unwrap_or_default()),
            ..Default::default()
        };

        if let Some(strategy_id) = self.model.strategy_id {
            active_model_action.strategy_id = ActiveValue::Set(strategy_id);
        }

        if let Some(is_active) = self.model.is_active {
            active_model_action.is_active = ActiveValue::Set(is_active);
        }

        if let Some(is_sell) = self.model.is_sell {
            active_model_action.is_sell = ActiveValue::Set(is_sell);
        }

        if let Some(is_quote_asset) = self.model.is_quote_asset {
            active_model_action.is_quote_asset = ActiveValue::Set(is_quote_asset);
        }

        if let Some(is_percentage) = self.model.is_percentage {
            active_model_action.is_percentage = ActiveValue::Set(is_percentage);
        }

        if let Some(value) = self.model.value {
            active_model_action.value = ActiveValue::Set(value);
        }

        if let Some(pair_id) = self.model.pair_id {
            active_model_action.pair_id = ActiveValue::Set(pair_id);
        }

        let now = Local::now();

        active_model_action.last_update = ActiveValue::Set(now.naive_local().into());

        match active_model_action.update(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn delete_action_data(self, db: &DatabaseConnection) -> Result<u64, Response> {
        match Entity::delete_by_id(self.model.id.unwrap_or_default())
            .exec(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val.rows_affected),
        }
    }
}
