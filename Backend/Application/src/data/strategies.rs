use models::entities::strategies::{self, ActiveModel, Column, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::{
    types::Strategies,
    utils::{Data, Outcome, OutcomeError},
};

impl Strategies<Data> {
    pub async fn insert_strategy(
        db: &DatabaseConnection,
        stragegies_type: Self,
    ) -> Outcome<Model, String, String> {
        let active_model_strategy = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(stragegies_type.model.name),
            is_active: ActiveValue::Set(stragegies_type.model.is_active),
            description: ActiveValue::Set(stragegies_type.model.description),
            can_trade: ActiveValue::Set(stragegies_type.model.can_trade),
            stream_name: ActiveValue::Set(stragegies_type.model.stream_name),
        };

        Entity::insert(active_model_strategy)
            .exec_with_returning(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }

    pub async fn select_strategy(
        db: &DatabaseConnection,
        id: i32,
    ) -> Outcome<Model, String, String> {
        Entity::find()
            .filter(Condition::all().add(Column::Id.eq(id)))
            .one(db)
            .await
            .map_err(|err| OutcomeError::Error(err.to_string()))?
            .ok_or_else(|| OutcomeError::Failure("Strategy not found".to_string()))
    }

    pub async fn select_active_strategies(
        db: &DatabaseConnection,
    ) -> Outcome<Vec<Model>, String, String> {
        let strategies = Entity::find()
            .filter(Condition::all().add(Column::IsActive.eq(true)))
            .all(db)
            .await
            .map_err(|err| OutcomeError::Error(err.to_string()))?;

        Outcome::Ok(strategies)
    }

    pub async fn update_strategy(
        db: &DatabaseConnection,
        stragegies_type: Self,
    ) -> Outcome<Model, String, String> {
        let mut strategy = strategies::ActiveModel {
            id: ActiveValue::Unchanged(stragegies_type.model.id),
            is_active: ActiveValue::Set(stragegies_type.model.is_active),
            can_trade: ActiveValue::Set(stragegies_type.model.can_trade),
            ..Default::default()
        };

        if !stragegies_type.model.name.is_empty() {
            strategy.name = ActiveValue::Set(stragegies_type.model.name);
        }

        if !stragegies_type.model.description.is_empty() {
            strategy.description = ActiveValue::set(stragegies_type.model.description);
        }

        if !stragegies_type.model.stream_name.is_empty() {
            strategy.stream_name = ActiveValue::set(stragegies_type.model.stream_name);
        }

        strategy
            .update(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }

    pub async fn delete_strategy(
        db: &DatabaseConnection,
        stragegies_type: Self,
    ) -> Outcome<u64, String, String> {
        Entity::delete_by_id(stragegies_type.model.id)
            .exec(db)
            .await
            .map(|val| Outcome::Ok(val.rows_affected))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }
}
