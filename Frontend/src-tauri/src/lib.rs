use crate::command::{
    delete_action, delete_indicator, delete_strategy, get_account, get_active_action,
    get_active_actions, get_active_indicator, get_active_indicators, get_active_metric,
    get_active_strategies, get_active_strategy, get_integration_setting, get_integrations_settings,
    get_subscribed_indicators, insert_action, insert_configuration, insert_indicator,
    insert_ledger, insert_order, insert_strategy, post_user_command, select_action, select_actions,
    select_active_tasks, select_assets, select_configuration, select_error_logs,
    select_health_check, select_indicator, select_indicators, select_integration_logs,
    select_integration_setting, select_integrations_settings, select_ledger, select_ledgers,
    select_metrics, select_order, select_orders, select_pairs, select_strategies,
    select_strategies_overview, select_strategy, select_task, select_tasks, set_engine_running,
    start_active_actions, start_active_indicators, start_active_strategies, start_active_tasks,
    stop_active_actions, stop_active_indicators, stop_active_strategies, stop_active_tasks,
    update_action, update_indicator, update_integration_setting, update_order, update_strategy,
    update_task,
};

pub mod command;
pub mod core;
pub mod integration;
pub mod models;

pub mod static_strings;
pub mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            select_health_check,
            set_engine_running,
            select_configuration,
            insert_configuration,
            select_assets,
            select_pairs,
            insert_strategy,
            select_strategy,
            select_strategies,
            update_strategy,
            delete_strategy,
            get_active_strategy,
            get_active_strategies,
            start_active_strategies,
            stop_active_strategies,
            insert_indicator,
            select_indicator,
            select_indicators,
            update_indicator,
            delete_indicator,
            get_active_indicator,
            get_active_indicators,
            get_subscribed_indicators,
            start_active_indicators,
            stop_active_indicators,
            select_action,
            select_actions,
            insert_action,
            update_action,
            delete_action,
            get_active_action,
            get_active_actions,
            start_active_actions,
            stop_active_actions,
            get_account,
            post_user_command,
            insert_ledger,
            select_ledger,
            select_ledgers,
            select_metrics,
            get_active_metric,
            insert_order,
            select_order,
            select_orders,
            update_order,
            select_error_logs,
            select_integration_logs,
            select_strategies_overview,
            select_task,
            select_tasks,
            update_task,
            select_active_tasks,
            start_active_tasks,
            stop_active_tasks,
            select_integration_setting,
            select_integrations_settings,
            update_integration_setting,
            get_integration_setting,
            get_integrations_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
