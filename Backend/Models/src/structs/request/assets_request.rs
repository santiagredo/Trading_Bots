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
}

impl AssetRequest {
    pub fn from_model(asset: &assets::Model) -> AssetRequest {
        AssetRequest {
            id: Some(asset.id),
            name: Some(asset.name.clone()),
            ticker: Some(asset.ticker.clone()),
            free: Some(asset.free),
            locked: Some(asset.locked),
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

    pub fn aggregate_values(mut self, value: Decimal, is_locked: bool, is_sell: bool) -> Self {
        match (is_locked, is_sell) {
            (true, true) => {
                if let Some(val) = self.locked.as_mut() {
                    *val -= value
                }
            }
            (true, false) => {
                if let Some(val) = self.locked.as_mut() {
                    *val += value
                }
            }
            (false, true) => {
                if let Some(val) = self.free.as_mut() {
                    *val -= value
                }
            }
            (false, false) => {
                if let Some(val) = self.free.as_mut() {
                    *val += value
                }
            }
        }

        self
    }
}
