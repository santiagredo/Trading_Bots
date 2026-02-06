use crate::{
    handler::StrategiesOverview,
    utils::{handle_db_error, Response},
};
use models::{
    entities::{
        actions, indicators, pairs,
        strategies::{self},
    },
    structs::{StrategyOverview, StrategyRequest},
};
use sea_orm::{ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};
use tracing::error_span;

impl StrategiesOverview {
    pub async fn select_strategy_overview(
        db: &DatabaseConnection,
        strategy: StrategyRequest,
    ) -> Result<StrategyOverview, Response> {
        let Some(strategy_id) = strategy.id else {
            return Err(Response::bad_request(format!("Strategy id is null")));
        };

        let mut strategy_overview = StrategyOverview::default();

        let strategy = match strategies::Entity::find_by_id(strategy_id).one(db).await {
            Err(err) => {
                // let _ = ErrorLogs::new(self.clone())
                //     .insert(log_trait_db_error!(err, req))
                //     .await;

                return Err(handle_db_error(&err));
            }
            Ok(None) => return Err(Response::not_found(format!("Strategy not found"))),
            Ok(Some(val)) => val,
        };

        strategy_overview.strategy = strategy;

        let indicator = match indicators::Entity::find()
            .filter(Condition::all().add(indicators::Column::StrategyId.eq(strategy_id)))
            .one(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => val,
        };

        strategy_overview.indicator = indicator.unwrap_or_default();

        let action = match actions::Entity::find()
            .filter(actions::Column::StrategyId.eq(strategy_id))
            .one(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => val,
        };

        strategy_overview.action = action.unwrap_or_default();

        let pair = match pairs::Entity::find()
            .filter(pairs::Column::Id.eq(strategy_overview.action.pair_id))
            .one(db)
            .await
        {
            Err(err) => {
                error_span!("error - database", error = ?err);

                return Err(handle_db_error(&err));
            }
            Ok(val) => val,
        };

        strategy_overview.pair = pair.unwrap_or_default();

        Ok(strategy_overview)
    }
}
