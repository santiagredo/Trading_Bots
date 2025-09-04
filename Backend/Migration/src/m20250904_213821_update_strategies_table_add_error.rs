use sea_orm_migration::prelude::*;

use crate::m20250121_024945_create_strategies_table::Strategies;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Strategies::Table)
                    .add_column(
                        ColumnDef::new(StrategiesError::ErrorCooldown)
                            .integer()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(StrategiesError::ErrorLastDate)
                            .date_time()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Strategies::Table)
                    .drop_column(StrategiesError::ErrorCooldown)
                    .drop_column(StrategiesError::ErrorLastDate)
                    .to_owned(),
            )
            .await
    }
}

#[derive(Iden)]
enum StrategiesError {
    ErrorCooldown,
    ErrorLastDate,
}
