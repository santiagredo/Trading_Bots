use models::{entities::actions::Model, structs::ActionRequest};

#[derive(Debug, Clone)]
pub struct Actions<R> {
    pub repo: R,
}

impl<R> Actions<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Actions<()> {
    pub fn blank() -> Actions<()> {
        Self { repo: () }
    }

    pub fn into_model(action: ActionRequest) -> Model {
        Model {
            id: action.id.unwrap_or_default(),
            strategy_id: action.strategy_id.unwrap_or_default(),
            is_active: action.is_active.unwrap_or_default(),
            is_sell: action.is_sell.unwrap_or_default(),
            is_quote_asset: action.is_quote_asset.unwrap_or_default(),
            is_percentage: action.is_percentage.unwrap_or_default(),
            value: action.value.unwrap_or_default(),
            pair_id: action.pair_id.unwrap_or_default(),
            last_update: action.last_update,
        }
    }
}
