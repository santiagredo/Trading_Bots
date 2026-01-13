use chrono::NaiveDateTime;
use sea_orm::prelude::Decimal;
use serde::{Deserialize, Serialize};

use crate::entities::assets;

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AssetRequest {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub ticker: Option<String>,
    pub free: Option<Decimal>,
    pub locked: Option<Decimal>,
    pub last_update: Option<NaiveDateTime>,
}

impl AssetRequest {
    pub fn from_model(asset: &assets::Model) -> AssetRequest {
        AssetRequest {
            id: Some(asset.id),
            name: Some(asset.name.clone()),
            ticker: Some(asset.ticker.clone()),
            free: Some(asset.free),
            locked: Some(asset.locked),
            last_update: asset.last_update,
        }
    }

    pub fn update_values(mut self, is_locked: bool, value: Decimal) -> Self {
        if is_locked {
            self.locked = Some(value);
        } else {
            self.free = Some(value);
        }

        self
    }
}
