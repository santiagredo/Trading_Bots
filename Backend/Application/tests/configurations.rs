use application::{handler::Configurations, utils::Cache};
use models::structs::Configuration;

#[tokio::test]
async fn full_configuration_cache_flow_should_work_correctly() {
    // Initial state
    let initial = Configurations::<Cache>::get_configuration_cache().await;
    assert!(initial.is_none());

    // Set configuration
    let config = Configuration::default();
    Configurations::<Cache>::set_configuration_cache(config.clone()).await;

    let cached = Configurations::<Cache>::get_configuration_cache().await;
    assert_eq!(cached, Some(config.clone()));

    // Overwrite configuration
    let new_config = Configuration {
        dev_database_url: "Test 1".to_string(),
        ..Default::default()
    };
    Configurations::<Cache>::set_configuration_cache(new_config.clone()).await;

    let cached_again = Configurations::<Cache>::get_configuration_cache().await;
    assert_eq!(cached_again, Some(new_config));
}
