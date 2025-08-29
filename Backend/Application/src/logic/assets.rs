use crate::{
    types::Assets,
    utils::{Logic, Utils},
};

impl Assets<Logic> {
    pub fn insert_asset_logic(mut self) -> Result<Self, String> {
        self.model.name = Some(
            Utils::validate_empty_field(self.model.name.clone().unwrap_or_default(), "Asset name")?
                .to_uppercase(),
        );

        self.model.ticker = Some(
            Utils::validate_empty_field(
                self.model.ticker.clone().unwrap_or_default(),
                "Asset ticker",
            )?
            .to_uppercase(),
        );

        Ok(self)
    }

    pub fn update_asset_logic(mut self) -> Result<Self, String> {
        if self.model.id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid asset ID"));
        }

        if let Some(name) = self.model.name.clone() {
            self.model.name = Some(Utils::validate_empty_field(name, "Asset name")?.to_uppercase());
        }

        if let Some(ticker) = self.model.ticker.clone() {
            self.model.ticker =
                Some(Utils::validate_empty_field(ticker, "Asset ticker")?.to_uppercase());
        }

        Ok(self)
    }

    pub fn delete_asset_logic(self) -> Result<Self, String> {
        if self.model.id.is_none_or(|id| id <= 0) {
            return Err(format!("Invalid asset ID"));
        }

        Ok(self)
    }
}

#[cfg(test)]
mod fn_insert_asset_logic {
    use super::*;
    use crate::utils::Core;
    use models::structs::AssetRequest;

    #[test]
    fn insert_asset_cases() {
        let cases = vec![
            (
                "ok_valid_insert",
                AssetRequest {
                    name: Some("Bitcoin".into()),
                    ticker: Some("btc".into()),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_missing_name",
                AssetRequest {
                    name: None,
                    ticker: Some("BTC".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_ticker",
                AssetRequest {
                    name: Some("Ethereum".into()),
                    ticker: None,
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let asset = Assets::new(req);

            let result = asset.next_phase::<Core>().next_phase().insert_asset_logic();

            assert_eq!(
                result.is_ok(),
                should_pass,
                "case `{}` failed: expected {}, got {:?}",
                name,
                should_pass,
                result
            );
        }
    }
}

#[cfg(test)]
mod fn_update_asset_logic {
    use super::*;
    use crate::utils::Core;
    use models::structs::AssetRequest;

    #[test]
    fn update_asset_cases() {
        let cases = vec![
            (
                "ok_valid_update",
                AssetRequest {
                    id: Some(1),
                    name: Some("Cardano".into()),
                    ticker: Some("ada".into()),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_id_zero",
                AssetRequest {
                    id: Some(0),
                    name: Some("Litecoin".into()),
                    ticker: Some("ltc".into()),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_id",
                AssetRequest {
                    id: None,
                    name: Some("XRP".into()),
                    ticker: Some("xrp".into()),
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let asset = Assets::new(req);

            let result = asset.next_phase::<Core>().next_phase().update_asset_logic();

            assert_eq!(
                result.is_ok(),
                should_pass,
                "case `{}` failed: expected {}, got {:?}",
                name,
                should_pass,
                result
            );
        }
    }
}

#[cfg(test)]
mod fn_delete_asset_logic {
    use super::*;
    use crate::utils::Core;
    use models::structs::AssetRequest;

    #[test]
    fn delete_asset_cases() {
        let cases = vec![
            (
                "ok_valid_delete",
                AssetRequest {
                    id: Some(10),
                    ..Default::default()
                },
                true,
            ),
            (
                "err_invalid_id_zero",
                AssetRequest {
                    id: Some(0),
                    ..Default::default()
                },
                false,
            ),
            (
                "err_missing_id",
                AssetRequest {
                    id: None,
                    ..Default::default()
                },
                false,
            ),
        ];

        for (name, req, should_pass) in cases {
            let asset = Assets::new(req);

            let result = asset.next_phase::<Core>().next_phase().delete_asset_logic();

            assert_eq!(
                result.is_ok(),
                should_pass,
                "case `{}` failed: expected {}, got {:?}",
                name,
                should_pass,
                result
            );
        }
    }
}
