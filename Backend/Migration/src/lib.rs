pub use sea_orm_migration::prelude::*;

mod m20250121_023349_create_status_table;
mod m20250121_024310_create_assets_table;
mod m20250121_024945_create_strategies_table;
mod m20250121_025306_create_orders_table;
mod m20250121_031110_create_record_types_table;
mod m20250121_031549_create_ledgers_table;
mod m20250130_013341_create_pair_assets_table;
mod m20250306_005527_create_strategies_pair_assets_table;

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
            Box::new(m20250130_013341_create_pair_assets_table::Migration),
            Box::new(m20250306_005527_create_strategies_pair_assets_table::Migration),
        ]
    }
}
