use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250121_024310_create_assets_table::Assets,
    m20250121_031110_create_record_types_table::RecordTypes,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Ledgers::Table)
                    .if_not_exists()
                    .col(pk_auto(Ledgers::Id))
                    .col(ColumnDef::new(Ledgers::OrderId).integer().null())
                    .col(integer(Ledgers::RecordTypeId))
                    .col(date_time(Ledgers::CreationDate))
                    .col(integer(Ledgers::AssetId))
                    .col(decimal_len(Ledgers::FreeAmount, 18, 8))
                    .col(decimal_len(Ledgers::FreePreviousBalance, 18, 8))
                    .col(decimal_len(Ledgers::FreeNewBalance, 18, 8))
                    .col(decimal_len(Ledgers::LockedAmount, 18, 8))
                    .col(decimal_len(Ledgers::LockedPreviousBalance, 18, 8))
                    .col(decimal_len(Ledgers::LockedNewBalance, 18, 8))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_ledgers_record_types")
                    .from(Ledgers::Table, Ledgers::RecordTypeId)
                    .to(RecordTypes::Table, RecordTypes::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_ledgers_assets")
                    .from(Ledgers::Table, Ledgers::AssetId)
                    .to(Assets::Table, Assets::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Ledgers::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Ledgers {
    Table,
    Id,
    OrderId,
    RecordTypeId,
    CreationDate,
    AssetId,
    FreeAmount,
    FreePreviousBalance,
    FreeNewBalance,
    LockedAmount,
    LockedPreviousBalance,
    LockedNewBalance,
}
