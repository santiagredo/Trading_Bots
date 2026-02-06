use application::handler::Configurations;
use models::structs::Configuration;

#[tokio::test]
async fn full_configuration_cache_flow_should_work_correctly() {
    // Initial state
    let initial = Configurations::get_configuration().await;
    assert!(initial.is_none());

    // Set configuration
    let config = Configuration::default();
    Configurations::set_configuration(config.clone()).await;

    let cached = Configurations::get_configuration().await;
    assert_eq!(cached, Some(config.clone()));

    // Overwrite configuration
    let new_config = Configuration {
        dev_database_url: "Test 1".to_string(),
        ..Default::default()
    };
    Configurations::set_configuration(new_config.clone()).await;

    let cached_again = Configurations::get_configuration().await;
    assert_eq!(cached_again, Some(new_config));
}
