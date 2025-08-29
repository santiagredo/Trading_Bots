use models::structs::CoinPaprikaTicker;
use tracing::error_span;

use crate::{types::CoinPaprika, utils::Core};

impl CoinPaprika<Core> {
    pub async fn get_tickers_core(self) -> Option<Vec<CoinPaprikaTicker>> {
        self.next_phase()
            .get_tickers_integration()
            .await
            .map_err(|err| {
                error_span!("Error - Binance", error = ?err);
                dbg!(eprint!("{err}"));
            })
            .ok()
    }
}
