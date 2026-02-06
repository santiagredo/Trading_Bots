use models::{entities::indicators::Model, structs::IndicatorRequest};

#[derive(Debug, Clone)]
pub struct Indicators<R> {
    pub repo: R,
}

impl<R> Indicators<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Indicators<()> {
    pub fn blank() -> Indicators<()> {
        Self { repo: () }
    }

    pub fn into_model(indicator: IndicatorRequest) -> Model {
        Model {
            id: indicator.id.unwrap_or_default(),
            strategy_id: indicator.strategy_id.unwrap_or_default(),
            is_active: indicator.is_active.unwrap_or_default(),
            symbol: indicator.symbol.unwrap_or_default(),
            nick: indicator.nick.unwrap_or_default(),
            direction: indicator.direction.unwrap_or_default(),
            is_percentage: indicator.is_percentage.unwrap_or_default(),
            value: indicator.value.unwrap_or_default(),
            last_update: indicator.last_update,
        }
    }
}
