use std::marker::PhantomData;

use models::entities::pair_assets::Model;

use crate::{
    types::PairAssets,
    utils::{Data, Logic},
};

impl PairAssets<Logic> {
    pub fn insert_pair_asset(model: Model) -> Result<PairAssets<Data>, String> {
        let invalid_fields: Vec<bool> = vec![
            model.base_asset_id == 0 || model.quote_asset_id == 0,
            model.symbol.is_empty(),
        ];

        if invalid_fields.iter().all(|item| *item) {
            return Err("Invalid parameters".to_owned());
        }

        Ok(PairAssets {
            phase: PhantomData::<Data>,
            model,
        })
    }

    pub fn select_pair_asset(model: Model) -> Result<PairAssets<Data>, String> {
        let invalid_fields: Vec<bool> = vec![
            model.id == 0,
            model.base_asset_id == 0 || model.quote_asset_id == 0,
            model.symbol.is_empty(),
        ];

        if invalid_fields.iter().all(|item| *item) {
            return Err("Invalid parameters".to_owned());
        }

        Ok(PairAssets {
            phase: PhantomData::<Data>,
            model,
        })
    }

    pub fn update_pair_asset(model: Model) -> Result<PairAssets<Data>, String> {
        let invalid_fields: Vec<bool> = vec![
            model.id == 0,
            model.base_asset_id == 0 || model.quote_asset_id == 0,
            model.symbol.is_empty(),
        ];

        if invalid_fields.iter().all(|item| *item) {
            return Err("Invalid parameters".to_owned());
        }

        Ok(PairAssets {
            phase: PhantomData::<Data>,
            model,
        })
    }
}
