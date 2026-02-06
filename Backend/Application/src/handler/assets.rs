use models::{entities::assets::Model, structs::AssetRequest};

#[derive(Debug, Clone)]
pub struct Assets<R> {
    pub repo: R,
}

impl<R> Assets<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }
}

impl Assets<()> {
    pub fn blank() -> Assets<()> {
        Self { repo: () }
    }

    pub fn into_request(model: Model) -> AssetRequest {
        AssetRequest {
            id: Some(model.id),
            name: Some(model.name),
            ticker: Some(model.ticker),
            free: Some(model.free),
            locked: Some(model.locked),
            last_update: model.last_update,
        }
    }

    pub fn into_model(asset: AssetRequest) -> Model {
        Model {
            id: asset.id.unwrap_or_default(),
            name: asset.name.unwrap_or_default(),
            ticker: asset.ticker.unwrap_or_default(),
            free: asset.free.unwrap_or_default(),
            locked: asset.locked.unwrap_or_default(),
            last_update: asset.last_update,
        }
    }
}
