use std::{sync::Arc, time::Duration};

use once_cell::sync::Lazy;
use tokio::{sync::RwLock, task::AbortHandle, time::sleep};

use crate::{
    handler::{Binance, CoinPaprika, Tasks},
    utils::Cache,
};

static TASKS_ABORT_HANDLES: Lazy<Arc<RwLock<Vec<AbortHandle>>>> =
    Lazy::new(|| Arc::new(RwLock::new(Vec::new())));

impl Tasks<Cache> {
    pub async fn start_async_tasks_cache() {
        let tasks_abort_handles = TASKS_ABORT_HANDLES.read().await;

        if !tasks_abort_handles.is_empty() {
            return;
        }

        // drop needed in order to prevent a deadlock
        drop(tasks_abort_handles);

        let account_information_handle = tokio::spawn(async move {
            loop {
                // Binance::update_account_balances().await;
                sleep(Duration::from_secs(86400)).await;
            }
        })
        .abort_handle();

        let exchange_information_handle = tokio::spawn(async move {
            loop {
                // let start = Instant::now();

                Binance::update_exchange_information().await;

                // let now = Local::now().naive_local();

                // dbg!(format!(
                //     "Update exchange information completed at {} -- duration of {:?}",
                //     now,
                //     start.elapsed()
                // ));

                sleep(Duration::from_secs(86400)).await;
            }
        })
        .abort_handle();

        let pair_statistics_handle = tokio::spawn(async move {
            loop {
                CoinPaprika::update_pairs_statistics().await;
                sleep(Duration::from_secs(300)).await;
            }
        })
        .abort_handle();

        // let metrics_handle = tokio::spawn(async move {
        //     loop {
        //         if let Some(mut metrics) = Metrics::get_active_metrics().await {
        //             for (metric_type, metric) in metrics.drain() {
        //                 dbg!(metric_type);
        //                 dbg!(metric.total_duration);
        //                 dbg!(metric.average().as_millis());
        //                 dbg!(metric.count);
        //                 dbg!(metric.fastest_duration);
        //                 dbg!(metric.slowest_duration);
        //             }
        //         }
        //         sleep(Duration::from_secs(1800)).await;
        //     }
        // })
        // .abort_handle();

        let vec_handles = vec![
            exchange_information_handle,
            pair_statistics_handle,
            // metrics_handle,
            account_information_handle,
        ];

        let mut tasks_abort_handles = TASKS_ABORT_HANDLES.write().await;

        *tasks_abort_handles = vec_handles;
    }

    pub async fn stop_async_tasks_cache() {
        let mut tasks_abort_handles = TASKS_ABORT_HANDLES.write().await;

        tasks_abort_handles.iter().for_each(|handle| handle.abort());

        *tasks_abort_handles = Vec::new();
    }
}
