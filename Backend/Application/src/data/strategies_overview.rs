use models::{
    entities::{
        actions, indicators, pairs,
        strategies::{self, Column},
    },
    structs::StrategyRequest,
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error_span;

use crate::{
    handler::StrategiesOverview,
    utils::{handle_db_error, Data, Response},
};

impl StrategiesOverview<Data> {
    pub async fn select_strategy_overview_data(
        db: &DatabaseConnection,
        strategy: StrategyRequest,
    ) -> Result<
        (
            Vec<strategies::Model>,
            Vec<indicators::Model>,
            Vec<actions::Model>,
            Vec<pairs::Model>,
        ),
        Response,
    > {
        let mut condition = Condition::all();

        if let Some(id) = strategy.id {
            condition = condition.add(Column::Id.eq(id));
        }

        if let Some(is_active) = strategy.is_active {
            condition = condition.add(Column::IsActive.eq(is_active));
        }

        let strategies = match strategies::Entity::find().filter(condition).all(db).await {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => val,
        };

        let indicators = match indicators::Entity::find()
            .filter(
                Condition::all().add(
                    indicators::Column::StrategyId
                        .is_in(strategies.iter().map(|strat| strat.id.clone())),
                ),
            )
            .all(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => val,
        };

        let actions = match actions::Entity::find()
            .filter(
                actions::Column::StrategyId.is_in(strategies.iter().map(|strat| strat.id.clone())),
            )
            .all(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => val,
        };

        let pairs = match pairs::Entity::find()
            .filter(pairs::Column::Id.is_in(actions.iter().map(|action| action.pair_id)))
            .all(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => val,
        };

        Ok((strategies, indicators, actions, pairs))
    }
}
