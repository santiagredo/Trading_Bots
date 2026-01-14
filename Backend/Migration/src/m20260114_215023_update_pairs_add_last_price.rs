use sea_orm_migration::prelude::*;

use crate::m20250130_013341_create_pairs_table::Pairs;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Pairs::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(PairsAlter::LastPrice)
                            .decimal()
                            .default(0)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Pairs::Table)
                    .drop_column(PairsAlter::LastPrice)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum PairsAlter {
    LastPrice,
}
