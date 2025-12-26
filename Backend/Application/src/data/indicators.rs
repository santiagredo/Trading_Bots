use function_name::named;
use models::{
    entities::indicators::{ActiveModel, Column, Entity, Model},
    structs::ErrorLogRequest,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::{
    handler::{ErrorLogs, Indicators},
    log_db_error,
    utils::{Data, Response},
};

impl Indicators<Data> {
    #[named]
    pub async fn insert_indicator_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let active_model_indicator = ActiveModel {
            id: ActiveValue::NotSet,
            strategy_id: ActiveValue::Set(self.model.strategy_id.unwrap_or_default()),
            is_active: ActiveValue::Set(self.model.is_active.unwrap_or_default()),
            symbol: ActiveValue::Set(self.model.symbol.clone().unwrap_or_default()),
            nick: ActiveValue::Set(self.model.nick.clone().unwrap_or_default()),
            direction: ActiveValue::Set(self.model.direction.clone().unwrap_or_default()),
            is_percentage: ActiveValue::Set(self.model.is_percentage.unwrap_or_default()),
            value: ActiveValue::Set(self.model.value.unwrap_or_default()),
        };

        match Entity::insert(active_model_indicator)
            .exec_with_returning(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_indicator_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Option<Model>, Response> {
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

        if let Some(symbol) = self.model.symbol.clone() {
            condition = condition.add(Column::Symbol.eq(symbol));
        }

        if let Some(nick) = self.model.nick.clone() {
            condition = condition.add(Column::Nick.eq(nick));
        }

        match Entity::find().filter(condition).one(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_indicators_data(
        self,
        db: &DatabaseConnection,
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

        if let Some(symbol) = self.model.symbol.clone() {
            condition = condition.add(Column::Symbol.eq(symbol));
        }

        if let Some(nick) = self.model.nick.clone() {
            condition = condition.add(Column::Nick.eq(nick));
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn update_indicator_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let mut active_model_indicator = ActiveModel {
            id: ActiveValue::Unchanged(self.model.id.unwrap_or_default()),
            ..Default::default()
        };

        if let Some(strategy_id) = self.model.strategy_id {
            active_model_indicator.strategy_id = ActiveValue::Set(strategy_id);
        }
        if let Some(is_active) = self.model.is_active {
            active_model_indicator.is_active = ActiveValue::Set(is_active);
        }
        if let Some(symbol) = self.model.symbol.clone() {
            active_model_indicator.symbol = ActiveValue::Set(symbol);
        }
        if let Some(nick) = self.model.nick.clone() {
            active_model_indicator.nick = ActiveValue::Set(nick);
        }
        if let Some(direction) = self.model.direction.clone() {
            active_model_indicator.direction = ActiveValue::Set(direction);
        }
        if let Some(is_percentage) = self.model.is_percentage {
            active_model_indicator.is_percentage = ActiveValue::Set(is_percentage);
        }
        if let Some(value) = self.model.value {
            active_model_indicator.value = ActiveValue::Set(value);
        }

        match active_model_indicator.update(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn delete_indicator_data(self, db: &DatabaseConnection) -> Result<u64, Response> {
        match Entity::delete_by_id(self.model.id.unwrap_or_default())
            .exec(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val.rows_affected),
        }
    }
}
