use std::collections::HashMap;

use models::{
    entities::{actions, indicators, pairs, strategies},
    structs::StrategyOverview,
};

use crate::{types::StrategiesOverview, utils::Logic};

impl StrategiesOverview<Logic> {
    pub fn select_strategies_overview_logic(
        results: (
            Vec<strategies::Model>,
            Vec<indicators::Model>,
            Vec<actions::Model>,
            Vec<pairs::Model>,
        ),
    ) -> Vec<StrategyOverview> {
        let (strategies, indicators, actions, pairs) = results;

        let mut indicators_map: HashMap<i32, indicators::Model> =
            indicators.into_iter().map(|i| (i.strategy_id, i)).collect();

        let mut actions_map: HashMap<i32, actions::Model> =
            actions.into_iter().map(|a| (a.strategy_id, a)).collect();

        let mut pairs_map: HashMap<i32, pairs::Model> =
            pairs.into_iter().map(|p| (p.id, p)).collect();

        let mut overview = vec![];

        for strategy in strategies {
            let indicator = indicators_map.remove(&strategy.id).unwrap_or_default();
            let action = actions_map.remove(&strategy.id).unwrap_or_default();
            let pair = pairs_map.remove(&action.pair_id).unwrap_or_default();

            overview.push(StrategyOverview {
                strategy,
                indicator,
                action,
                pair,
                ..Default::default()
            });
        }

        overview
    }
}
