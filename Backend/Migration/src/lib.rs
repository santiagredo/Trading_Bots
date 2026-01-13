pub use sea_orm_migration::prelude::*;

mod m20250121_023349_create_status_table;
mod m20250121_024310_create_assets_table;
mod m20250121_024945_create_strategies_table;
mod m20250121_025306_create_orders_table;
mod m20250121_031110_create_record_types_table;
mod m20250121_031549_create_ledgers_table;
mod m20250130_013341_create_pairs_table;
mod m20250625_024136_create_indicators_table;
mod m20250625_030535_create_actions_table;
mod m20250904_011953_create_tasks_table;
mod m20251106_014805_create_integration_log_table;
mod m20251106_185615_create_error_log_table;
mod m20251222_234655_create_metrics_table;
mod m20260113_191430_update_assets_add_last_update;
mod m20260113_191948_update_tasks_add_last_update;
mod m20260113_194125_update_tasks_add_last_execution;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250121_023349_create_status_table::Migration),
            Box::new(m20250121_024310_create_assets_table::Migration),
            Box::new(m20250121_024945_create_strategies_table::Migration),
            Box::new(m20250121_025306_create_orders_table::Migration),
            Box::new(m20250121_031110_create_record_types_table::Migration),
            Box::new(m20250121_031549_create_ledgers_table::Migration),
            Box::new(m20250130_013341_create_pairs_table::Migration),
            Box::new(m20250625_024136_create_indicators_table::Migration),
            Box::new(m20250625_030535_create_actions_table::Migration),
            Box::new(m20250904_011953_create_tasks_table::Migration),
            Box::new(m20251106_014805_create_integration_log_table::Migration),
            Box::new(m20251106_185615_create_error_log_table::Migration),
            Box::new(m20251222_234655_create_metrics_table::Migration),
            Box::new(m20260113_191430_update_assets_add_last_update::Migration),
            Box::new(m20260113_191948_update_tasks_add_last_update::Migration),
            Box::new(m20260113_194125_update_tasks_add_last_execution::Migration),
        ]
    }
}
