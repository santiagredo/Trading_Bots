use std::marker::PhantomData;

use chrono::DateTime;
use models::structs::{CoinPaprikaTicker, Environments, Quote};
use sea_orm::prelude::Decimal;
use tracing::error_span;

use crate::{handler::Pairs, utils::Types};

pub struct CoinPaprika<Phase = Types> {
    phase: PhantomData<Phase>,
    pub environment: Environments,
}

impl CoinPaprika {
    pub fn default() -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment: Environments::DEV,
        }
    }

    pub fn with_env(self, environment: Environments) -> Self {
        Self {
            phase: PhantomData::<Types>,
            environment,
        }
    }

    pub async fn update_pairs_statistics(self) {
        // let start = Instant::now();
        let environment = self.environment;

        let mut stored_pairs = match Pairs::default().with_env(environment).select_pairs().await {
            Ok(pairs) if !pairs.is_empty() => pairs,
            _ => return,
        };

        let coinpaprika_tickers = match Self::default().get_tickers().await {
            Ok(tickers) if !tickers.is_empty() => tickers,
            _ => return,
        };

        let updates: Vec<(String, Quote)> = coinpaprika_tickers
            .into_iter()
            .flat_map(|ticker| {
                let ticker_symbol = ticker.symbol.to_uppercase();

                ticker
                    .quotes
                    .into_iter()
                    .map(move |(quote_key, quote_value)| {
                        let mapped_key = if quote_key == "USD" {
                            "USDT".to_string()
                        } else {
                            quote_key.to_uppercase()
                        };

                        let full_symbol = format!("{}{}", ticker_symbol, mapped_key);

                        (full_symbol, quote_value)
                    })
            })
            .collect();

        for pair in stored_pairs.iter_mut() {
            if let Some((_, quote)) = updates
                .iter()
                .find(|(full_symbol, _)| full_symbol == &pair.symbol)
            {
                // let span =
                //     warn_span!("CoinPaprika - Pair - Update", symbol = ?pair, quote = ?quote);

                // async move {
                // Update ATH-related fields.
                pair.all_time_high_price =
                    Decimal::from_f64_retain(quote.ath_price).unwrap_or_default();

                pair.all_time_high_date = DateTime::parse_from_rfc3339(&quote.ath_date)
                    .map(|dt| dt.naive_utc())
                    .unwrap_or_default();

                pair.percent_from_all_time_high =
                    Decimal::from_f64_retain(quote.percent_from_price_ath).unwrap_or_default();

                // Update price percent change fields.
                pair.fifteen_minutes_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_15_m).unwrap_or_default();

                pair.thirty_minutes_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_30_m).unwrap_or_default();

                pair.hour_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_1_h).unwrap_or_default();

                pair.six_hours_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_6_h).unwrap_or_default();

                pair.twelve_hours_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_12_h).unwrap_or_default();

                pair.day_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_24_h).unwrap_or_default();

                pair.week_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_7_d).unwrap_or_default();

                pair.month_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_30_d).unwrap_or_default();

                pair.year_price_percent_change =
                    Decimal::from_f64_retain(quote.percent_change_1_y).unwrap_or_default();

                let model = match Pairs::default()
                    .with_env(environment)
                    .from_model(pair.clone())
                    .update_pair()
                    .await
                {
                    Err(err) => {
                        error_span!("CoinPaprika - Pair - Update - Error", pair = ?pair, quote = ?quote, error = ?err);
                        dbg!(eprint!("{err:?} \n"));
                        continue;
                    }
                    Ok(val) => val,
                };

                Pairs::default()
                    .with_env(environment)
                    .from_model(model)
                    .set_active_pair(false)
                    .await;
            }
        }
    }
}

impl<Phase> CoinPaprika<Phase> {
    pub fn next_phase<Next>(self) -> CoinPaprika<Next> {
        CoinPaprika {
            phase: PhantomData::<Next>,
            environment: self.environment,
        }
    }
}

impl CoinPaprika<Types> {
    pub async fn get_tickers(self) -> Result<Vec<CoinPaprikaTicker>, String> {
        self.next_phase().get_tickers_core().await
    }
}
