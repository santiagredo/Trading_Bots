use chrono::Local;
use function_name::named;
use models::{
    entities::strategies::{self, ActiveModel, Column, Entity, Model},
    structs::ErrorLogRequest,
};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::{
    handler::{ErrorLogs, Strategies},
    log_db_error,
    utils::{Data, Response},
};

impl Strategies<Data> {
    #[named]
    pub async fn insert_strategy_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let now = Local::now();

        let active_model_strategy = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(self.model.name.clone().unwrap_or_default()),
            is_active: ActiveValue::Set(self.model.is_active.unwrap_or_default()),
            description: ActiveValue::Set(self.model.description.clone()),
            can_trade: ActiveValue::Set(self.model.can_trade.unwrap_or_default()),
            last_execution: ActiveValue::Set(self.model.last_execution),
            cooldown: ActiveValue::Set(self.model.cooldown),
            error_cooldown: ActiveValue::Set(self.model.error_cooldown),
            error_last_date: ActiveValue::Set(self.model.error_last_date),
            last_update: ActiveValue::Set(now.naive_local().into()),
        };

        match Entity::insert(active_model_strategy)
            .exec_with_returning(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_strategy_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Option<Model>, Response> {
        match Entity::find()
            .filter(Condition::all().add(Column::Id.eq(self.model.id)))
            .one(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn select_strategies_data(
        self,
        db: &DatabaseConnection,
    ) -> Result<Vec<Model>, Response> {
        let mut condition = Condition::all();

        if let Some(id) = self.model.id {
            condition = condition.add(Column::Id.eq(id))
        }

        if let Some(name) = self.model.name.as_ref() {
            condition = condition.add(Column::Name.eq(name.clone()))
        }

        if let Some(is_active) = self.model.is_active {
            condition = condition.add(Column::IsActive.eq(is_active))
        }

        if let Some(can_trade) = self.model.can_trade {
            condition = condition.add(Column::CanTrade.eq(can_trade))
        }

        if let Some(description) = self.model.description.as_ref() {
            condition = condition.add(Column::Description.eq(description.clone()))
        }

        if let Some(last_execution) = self.model.last_execution {
            condition = condition.add(Column::LastExecution.eq(last_execution))
        }

        if let Some(cooldown) = self.model.cooldown {
            condition = condition.add(Column::Cooldown.eq(cooldown))
        }

        if let Some(error_cooldown) = self.model.error_cooldown {
            condition = condition.add(Column::ErrorCooldown.eq(error_cooldown))
        }

        if let Some(error_last_date) = self.model.error_last_date {
            condition = condition.add(Column::ErrorLastDate.eq(error_last_date))
        }

        match Entity::find().filter(condition).all(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn update_strategy_data(self, db: &DatabaseConnection) -> Result<Model, Response> {
        let mut strategy = strategies::ActiveModel {
            id: ActiveValue::Unchanged(self.model.id.unwrap_or_default()),
            ..Default::default()
        };

        if let Some(name) = self.model.name.as_ref() {
            strategy.name = ActiveValue::Set(name.clone());
        }

        if let Some(is_active) = self.model.is_active {
            strategy.is_active = ActiveValue::Set(is_active);
        }

        if let Some(can_trade) = self.model.can_trade {
            strategy.can_trade = ActiveValue::Set(can_trade);
        }

        if let Some(description) = self.model.description.as_ref() {
            strategy.description = ActiveValue::Set(Some(description.clone()));
        }

        if let Some(last_execution) = self.model.last_execution {
            strategy.last_execution = ActiveValue::Set(Some(last_execution));
        }

        if let Some(cooldown) = self.model.cooldown {
            strategy.cooldown = ActiveValue::Set(Some(cooldown))
        }

        if let Some(error_cooldown) = self.model.error_cooldown {
            strategy.error_cooldown = ActiveValue::Set(Some(error_cooldown))
        }

        if let Some(error_last_date) = self.model.error_last_date {
            strategy.error_last_date = ActiveValue::Set(Some(error_last_date))
        }

        if self.model.last_update.is_none() {
            let now = Local::now();

            strategy.last_update = ActiveValue::Set(now.naive_local().into());
        }

        match strategy.update(db).await {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val),
        }
    }

    #[named]
    pub async fn delete_strategy_data(self, db: &DatabaseConnection) -> Result<u64, Response> {
        match Entity::delete_by_id(self.model.id.unwrap_or_default())
            .exec(db)
            .await
        {
            Err(err) => log_db_error!(self, err),
            Ok(val) => Ok(val.rows_affected),
        }
    }
}
