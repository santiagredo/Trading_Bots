use models::entities::{
    strategies,
    strategies_pair_assets::{Column, Entity, Model},
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    types::StrategiesPairAssets,
    utils::{Data, Outcome, OutcomeError},
};

impl StrategiesPairAssets<Data> {
    pub async fn select_active_strategies_pair_assets(
        db: &DatabaseConnection,
        active_strategies: &Vec<strategies::Model>,
    ) -> Outcome<Vec<Model>, String, String> {
        let strategies_ids = active_strategies
            .iter()
            .map(|val| val.id)
            .collect::<Vec<i32>>();

        let strats_pair_assets = Entity::find()
            .filter(Condition::all().add(Column::StrategyId.is_in(strategies_ids)))
            .all(db)
            .await
            .map_err(|err| OutcomeError::Error(err.to_string()))?;

        Ok(strats_pair_assets)
    }
}
