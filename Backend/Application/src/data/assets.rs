use models::entities::assets::{self, ActiveModel, Column, Entity, Model};
use sea_orm::{
    ActiveModelTrait, ActiveValue, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter,
};

use crate::{
    types::Assets,
    utils::{Data, Outcome, OutcomeError},
};

impl Assets<Data> {
    pub async fn insert_asset(
        db: &DatabaseConnection,
        asset_type: Self,
    ) -> Outcome<Model, String, String> {
        let active_model_asset = ActiveModel {
            id: ActiveValue::NotSet,
            name: ActiveValue::Set(asset_type.model.name),
            ticker: ActiveValue::Set(asset_type.model.ticker),
            free: ActiveValue::Set(asset_type.model.free),
            locked: ActiveValue::Set(asset_type.model.locked),
        };

        Entity::insert(active_model_asset)
            .exec_with_returning(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }

    pub async fn select_asset(
        db: &DatabaseConnection,
        asset_type: Self,
    ) -> Outcome<Model, String, String> {
        let mut condition = Condition::all();

        if asset_type.model.id != 0 {
            condition = condition.add(Column::Id.eq(asset_type.model.id))
        }

        if !asset_type.model.name.is_empty() {
            condition = condition.add(Column::Name.eq(asset_type.model.name))
        }

        if !asset_type.model.ticker.is_empty() {
            condition = condition.add(Column::Ticker.eq(asset_type.model.ticker))
        }

        Entity::find()
            .filter(condition)
            .one(db)
            .await
            .map_err(|err| OutcomeError::Error(err.to_string()))?
            .ok_or_else(|| OutcomeError::Failure("Asset not found".to_string()))
    }

    pub async fn select_assets(db: &DatabaseConnection) -> Outcome<Vec<Model>, String, String> {
        let assets = Entity::find()
            .all(db)
            .await
            .map_err(|err| OutcomeError::Error(err.to_string()))?;

        Outcome::Ok(assets)
    }

    pub async fn update_asset(
        db: &DatabaseConnection,
        asset_type: Self,
    ) -> Outcome<Model, String, String> {
        let mut active_model_asset = assets::ActiveModel {
            id: ActiveValue::Unchanged(asset_type.model.id),
            free: ActiveValue::Set(asset_type.model.free),
            locked: ActiveValue::Set(asset_type.model.locked),
            ..Default::default()
        };

        if !asset_type.model.name.is_empty() {
            active_model_asset.name = ActiveValue::Set(asset_type.model.name);
        }

        if !asset_type.model.ticker.is_empty() {
            active_model_asset.ticker = ActiveValue::set(asset_type.model.ticker);
        }

        active_model_asset
            .update(db)
            .await
            .map(|val| Outcome::Ok(val))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }

    pub async fn delete_asset(
        db: &DatabaseConnection,
        asset_type: Self,
    ) -> Outcome<u64, String, String> {
        Entity::delete_by_id(asset_type.model.id)
            .exec(db)
            .await
            .map(|val| Outcome::Ok(val.rows_affected))
            .map_err(|err| OutcomeError::Error(err.to_string()))?
    }
}
