use function_name::named;
use models::{
    entities::actions::{ActiveModel, Column, Entity, Model},
    structs::ErrorLogRequest,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::{
    handler::{Actions, ErrorLogs},
    log_db_error,
    utils::{Data, Response},
};

impl Actions<Data> {
    #[named]
    pub async fn insert_action_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let active_model_action = ActiveModel {
            id: ActiveValue::NotSet,
            strategy_id: ActiveValue::Set(self.model.strategy_id.unwrap_or_default()),
            is_active: ActiveValue::Set(self.model.is_active.unwrap_or_default()),
            is_sell: ActiveValue::Set(self.model.is_sell.unwrap_or_default()),
            is_quote_asset: ActiveValue::Set(self.model.is_quote_asset.unwrap_or_default()),
            is_percentage: ActiveValue::Set(self.model.is_percentage.unwrap_or_default()),
            value: ActiveValue::Set(self.model.value.unwrap_or_default()),
            pair_id: ActiveValue::Set(self.model.pair_id.unwrap_or_default()),
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
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if self.model.id.is_some() {
            condition = condition.add(Column::Id.eq(self.model.id.unwrap_or_default()))
        }

        if self.model.strategy_id.is_some() {
            condition =
                condition.add(Column::StrategyId.eq(self.model.strategy_id.unwrap_or_default()))
        }

        if self.model.is_active.is_some() {
            condition = condition.add(Column::IsActive.eq(self.model.is_active.unwrap_or_default()))
        }

        if self.model.pair_id.is_some() {
            condition = condition.add(Column::PairId.eq(self.model.pair_id.unwrap_or_default()))
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
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

    // pub async fn select_actions_by_strategy_id(
    //     db: &DatabaseConnection,
    //     id: i32,
    // ) -> Outcome<Vec<Model>, String, String> {
    //     Entity::find()
    //         .filter(Column::StrategyId.eq(id))
    //         .all(db)
    //         .await
    //         .map(|val| Outcome::Ok(val))
    //         .map_err(|err| OutcomeError::Error(err.to_string()))?
    // }

    // pub async fn select_actions_by_strategies_id(
    //     db: &DatabaseConnection,
    //     strategies_ids: Vec<i32>,
    // ) -> Outcome<Vec<Model>, String, String> {
    //     Entity::find()
    //         .filter(Column::StrategyId.is_in(strategies_ids))
    //         .all(db)
    //         .await
    //         .map(|val| Outcome::Ok(val))
    //         .map_err(|err| OutcomeError::Error(err.to_string()))?
    // }
}
