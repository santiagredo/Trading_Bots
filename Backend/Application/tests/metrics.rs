use std::time::Duration;

use application::{handler::Metrics, utils::EntityCache};
use models::{enums::LifecycleState, structs::Environments};

#[tokio::test]
async fn full_metrics_cache_flow_should_work_correctly() {
    let env = Environments::DEV;
    let metrics = Metrics::blank();

    /* ===========================
     * INITIAL STATE
     * ===========================
     */

    // Off => cache inaccesible
    assert!(metrics.get_all(env).await.is_none());

    /* ===========================
     * START CACHE
     * ===========================
     */

    metrics
        .set_state(env, LifecycleState::Starting)
        .await
        .unwrap();

    metrics
        .set_state(env, LifecycleState::Running)
        .await
        .unwrap();

    /* ===========================
     * EXECUTIONS
     * ===========================
     */

    metrics
        .set_execution_metrics(env, Duration::from_millis(100), true)
        .await;

    metrics
        .set_execution_metrics(env, Duration::from_millis(200), false)
        .await;

    /* ===========================
     * POSTING
     * ===========================
     */

    metrics.set_posting_metrics(env, true).await;
    metrics.set_posting_metrics(env, false).await;

    /* ===========================
     * SKIPPED
     * ===========================
     */

    metrics.set_skipped_metrics(env).await;

    /* ===========================
     * VALIDATE CACHE
     * ===========================
     */

    let cache = metrics.get_all(env).await.unwrap();
    let metric = cache.model;

    assert_eq!(metric.executions_ok, 1);
    assert_eq!(metric.executions_err, 1);
    assert_eq!(metric.consecutive_errors, 1);
    assert_eq!(metric.max_execution_time, 200);
    assert_eq!(metric.active_posting, 0);
    assert_eq!(metric.skipped_due_to_lock, 1);

    /* ===========================
     * STOP
     * ===========================
     */

    metrics
        .set_state(env, LifecycleState::Stopping)
        .await
        .unwrap();

    let removed = metrics.remove_all(env).await.unwrap();
    assert_eq!(removed.len(), 1);

    metrics.set_state(env, LifecycleState::Off).await.unwrap();

    // Off => cache inaccesible
    assert!(metrics.get_all(env).await.is_none());
}
