// use application::{handler::Indicators, utils::Cache};
// use models::{entities::indicators::Model, enums::LifecycleState, structs::Environments};

// fn mock_indicator(id: i32) -> Model {
//     Model {
//         id,
//         strategy_id: id,
//         is_active: true,
//         ..Default::default()
//     }
// }

// #[tokio::test]
// async fn full_indicators_cache_flow_should_work_correctly() {
//     let env = Environments::DEV;

//     /* ===========================
//      * INITIAL STATE
//      * ===========================
//      */

//     let reset = Indicators::<Cache>::reset_indicators_cache(env).await;
//     assert!(reset.is_ok());

//     let off = Indicators::<Cache>::set_status_cache(env, LifecycleState::Off).await;
//     assert!(off.is_err());

//     let status = Indicators::<Cache>::get_cache_state(env).await;
//     assert_eq!(status, LifecycleState::Off);

//     let cache = Indicators::<Cache>::get_indicators_cache(env).await;
//     assert!(cache.is_some_and(|val| val.models.is_empty()));

//     /* ===========================
//      * LOAD MULTIPLE INDICATORS
//      * ===========================
//      */

//     let starting = Indicators::<Cache>::set_status_cache(env, LifecycleState::Starting).await;
//     assert!(starting.is_ok());

//     let indicators = vec![mock_indicator(1), mock_indicator(2)];

//     let running = Indicators::<Cache>::set_status_cache(env, LifecycleState::Running).await;
//     assert!(running.is_ok());

//     Indicators::<Cache>::set_indicators_cache(env, indicators)
//         .await
//         .unwrap();

//     let cache = Indicators::<Cache>::get_indicators_cache(env)
//         .await
//         .unwrap();
//     assert_eq!(cache.models.len(), 2);
//     assert_eq!(cache.status, LifecycleState::Running);

//     /* ===========================
//      * INSERT INDIVIDUAL INDICATOR
//      * ===========================
//      */

//     let extra = mock_indicator(3);
//     Indicators::<Cache>::upsert_indicator_cache(env, extra.clone())
//         .await
//         .unwrap();

//     let single = Indicators::<Cache>::get_indicator_cache(env, 3)
//         .await
//         .unwrap();
//     assert_eq!(single, extra);

//     /* ===========================
//      * STOP
//      * ===========================
//      */

//     Indicators::<Cache>::set_status_cache(env, LifecycleState::Stopping)
//         .await
//         .unwrap();

//     Indicators::<Cache>::remove_indicators_cache(env)
//         .await
//         .unwrap();

//     Indicators::<Cache>::set_status_cache(env, LifecycleState::Off)
//         .await
//         .unwrap();

//     let final_cache = Indicators::<Cache>::get_indicators_cache(env).await;
//     assert!(final_cache.is_some_and(|val| val.models.is_empty()));
// }
