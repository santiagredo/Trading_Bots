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
                    .col(integer(Ledgers::BaseAssetId))
                    .col(decimal_len(Ledgers::BaseAssetAmount, 18, 8))
                    .col(decimal_len(Ledgers::BaseAssetPreviousBalance, 18, 8))
                    .col(decimal_len(Ledgers::BaseAssetNewBalance, 18, 8))
                    .col(integer(Ledgers::QuoteAssetId))
                    .col(decimal_len(Ledgers::QuoteAssetAmount, 18, 8))
                    .col(decimal_len(Ledgers::QuoteAssetPreviousBalance, 18, 8))
                    .col(decimal_len(Ledgers::QuoteAssetNewBalance, 18, 8))
                    .to_owned(),
            )
            .await?;

        // manager
        //     .create_foreign_key(
        //         ForeignKey::create()
        //             .name("fk_ledgers_orders")
        //             .from(Ledgers::Table, Ledgers::OrderId)
        //             .to(Orders::Table, Orders::Id)
        //             .to_owned(),
        //     )
        //     .await?;

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
                    .name("fk_ledgers_assets-base")
                    .from(Ledgers::Table, Ledgers::BaseAssetId)
                    .to(Assets::Table, Assets::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_ledgers_assets-quote")
                    .from(Ledgers::Table, Ledgers::QuoteAssetId)
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
    BaseAssetId,
    BaseAssetAmount,
    BaseAssetPreviousBalance,
    BaseAssetNewBalance,
    QuoteAssetId,
    QuoteAssetAmount,
    QuoteAssetPreviousBalance,
    QuoteAssetNewBalance,
}
