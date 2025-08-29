use models::entities::assets;

use crate::{types::Pairs, utils::Logic};

impl Pairs<Logic> {
    pub fn insert_pair_logic(
        mut self,
        base_asset: Option<assets::Model>,
        quote_asset: Option<assets::Model>,
    ) -> Result<Self, String> {
        if base_asset.as_ref().is_none_or(|model| model.id == 0) {
            return Err("Invalid base asset".to_owned());
        }

        if quote_asset.as_ref().is_none_or(|model| model.id == 0) {
            return Err("Invalid quote asset".to_owned());
        }

        self.model.symbol = Some(format!(
            "{}{}",
            base_asset.unwrap_or_default().ticker,
            quote_asset.unwrap_or_default().ticker
        ));

        Ok(self)
    }

    pub fn update_pair_logic(
        self,
        base_asset: Option<assets::Model>,
        quote_asset: Option<assets::Model>,
    ) -> Result<Self, String> {
        if self.model.id.is_none_or(|id| id <= 0) {
            return Err("Invalid pair id".to_owned());
        }

        if base_asset.is_none_or(|model| model.id == 0) {
            return Err("Invalid base asset".to_owned());
        }

        if quote_asset.is_none_or(|model| model.id == 0) {
            return Err("Invalid quote asset".to_owned());
        }

        Ok(self)
    }
}

#[cfg(test)]
mod fn_insert_pair_logic {
    use crate::types::Pairs;
    use crate::utils::Core;
    use models::entities::assets;

    #[test]
    fn insert_pair_cases() {
        let cases = vec![
            (
                "ok_valid_assets",
                Some(assets::Model {
                    id: 1,
                    ticker: "BTC".into(),
                    ..Default::default()
                }),
                Some(assets::Model {
                    id: 2,
                    ticker: "USDT".into(),
                    ..Default::default()
                }),
                true,
            ),
            (
                "err_invalid_base",
                Some(assets::Model {
                    id: 0,
                    ticker: "BTC".into(),
                    ..Default::default()
                }),
                Some(assets::Model {
                    id: 2,
                    ticker: "USDT".into(),
                    ..Default::default()
                }),
                false,
            ),
            (
                "err_invalid_quote",
                Some(assets::Model {
                    id: 1,
                    ticker: "BTC".into(),
                    ..Default::default()
                }),
                Some(assets::Model {
                    id: 0,
                    ticker: "USDT".into(),
                    ..Default::default()
                }),
                false,
            ),
            (
                "err_missing_base",
                None,
                Some(assets::Model {
                    id: 2,
                    ticker: "USDT".into(),
                    ..Default::default()
                }),
                false,
            ),
            (
                "err_missing_quote",
                Some(assets::Model {
                    id: 1,
                    ticker: "BTC".into(),
                    ..Default::default()
                }),
                None,
                false,
            ),
        ];

        for (name, base, quote, should_pass) in cases {
            let pair = Pairs::default();

            let result = pair
                .next_phase::<Core>()
                .next_phase()
                .insert_pair_logic(base, quote);

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
mod fn_update_pair_logic {
    use crate::types::Pairs;
    use crate::utils::Core;
    use models::entities::assets;

    #[test]
    fn update_pair_cases() {
        let cases = vec![
            (
                "ok_valid_update",
                1,
                Some(assets::Model {
                    id: 1,
                    ticker: "ETH".into(),
                    ..Default::default()
                }),
                Some(assets::Model {
                    id: 2,
                    ticker: "USDT".into(),
                    ..Default::default()
                }),
                true,
            ),
            (
                "err_invalid_id",
                0,
                Some(assets::Model {
                    id: 1,
                    ticker: "ETH".into(),
                    ..Default::default()
                }),
                Some(assets::Model {
                    id: 2,
                    ticker: "USDT".into(),
                    ..Default::default()
                }),
                false,
            ),
            (
                "err_invalid_base",
                1,
                Some(assets::Model {
                    id: 0,
                    ticker: "ETH".into(),
                    ..Default::default()
                }),
                Some(assets::Model {
                    id: 2,
                    ticker: "USDT".into(),
                    ..Default::default()
                }),
                false,
            ),
            (
                "err_invalid_quote",
                1,
                Some(assets::Model {
                    id: 1,
                    ticker: "ETH".into(),
                    ..Default::default()
                }),
                Some(assets::Model {
                    id: 0,
                    ticker: "USDT".into(),
                    ..Default::default()
                }),
                false,
            ),
        ];

        for (name, id, base, quote, should_pass) in cases {
            let mut pair = Pairs::default();
            pair.model.id = Some(id);

            let result = pair
                .next_phase::<Core>()
                .next_phase()
                .update_pair_logic(base, quote);

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
