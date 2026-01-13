use std::time::Duration;

use application::{handler::Metrics, utils::Cache};
use models::structs::Environments;

#[tokio::test]
async fn full_metrics_cache_flow_should_work_correctly() {
    let env = Environments::DEV;
    let metrics = Metrics::default().with_env(env).next_phase::<Cache>();

    // Initial state
    assert!(metrics.get_metrics_cache().await.is_none());

    // Successful execution
    Metrics::<Cache>::set_execution_metrics_cache(env, Duration::from_millis(100), true)
        .await;

    // Error execution
    Metrics::<Cache>::set_execution_metrics_cache(env, Duration::from_millis(200), false)
        .await;

    // Posting activity
    Metrics::<Cache>::set_posting_metrics_cache(env, true).await;
    Metrics::<Cache>::set_posting_metrics_cache(env, false).await;

    // Skipped
    Metrics::<Cache>::set_skipped_metrics_cache(env).await;

    let metrics = Metrics::default().with_env(env).next_phase::<Cache>();

    let metric = metrics.get_metrics_cache().await.unwrap();

    assert_eq!(metric.executions_ok, 1);
    assert_eq!(metric.executions_err, 1);
    assert_eq!(metric.consecutive_errors, 1);
    assert_eq!(metric.max_execution_time, Duration::from_millis(200));
    assert_eq!(metric.active_posting, 0);
    assert_eq!(metric.skipped_due_to_lock, 1);

    // Persist
    let persisted = Metrics::<Cache>::persist_metrics_cache(env).await;
    assert!(persisted.is_some());

    let metrics = Metrics::default().with_env(env).next_phase::<Cache>();

    // Cache cleared
    assert!(metrics.get_metrics_cache().await.is_none());
}
