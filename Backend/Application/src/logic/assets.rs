use std::marker::PhantomData;

use models::entities::assets::Model;

use crate::{
    types::Assets,
    utils::{Data, Logic, Utils},
};

impl Assets<Logic> {
    pub fn insert_asset(mut asset: Model) -> Result<Assets<Data>, String> {
        asset.name = Utils::validate_empty_field(asset.name.clone(), "Asset name")?;
        asset.ticker = Utils::validate_empty_field(asset.ticker.clone(), "Asset ticker")?;

        Ok(Assets {
            phase: PhantomData::<Data>,
            model: asset,
        })
    }

    pub fn select_asset(asset: Model) -> Result<Assets<Data>, String> {
        let invalid_fields: Vec<bool> = vec![
            asset.id == 0,
            asset.name.is_empty(),
            asset.ticker.is_empty(),
        ];

        if invalid_fields.iter().all(|item| *item) {
            return Err("Invalid parameters".to_owned());
        }

        Ok(Assets {
            phase: PhantomData::<Data>,
            model: asset,
        })
    }

    pub fn select_assets() -> Result<(), ()> {
        Ok(())
    }

    pub fn update_asset(asset: Model) -> Result<Assets<Data>, String> {
        if asset.id <= 0 {
            return Err(format!("Invalid asset ID"));
        }

        Ok(Assets {
            phase: PhantomData::<Data>,
            model: asset,
        })
    }

    pub fn delete_asset(asset: Model) -> Result<Assets<Data>, String> {
        if asset.id <= 0 {
            return Err(format!("Invalid asset ID"));
        }

        Ok(Assets {
            phase: PhantomData::<Data>,
            model: asset,
        })
    }
}
