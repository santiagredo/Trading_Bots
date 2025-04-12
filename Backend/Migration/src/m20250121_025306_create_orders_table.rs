use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250121_023349_create_status_table::Status, m20250121_024310_create_assets_table::Assets,
    m20250121_024945_create_strategies_table::Strategies,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Orders::Table)
                    .if_not_exists()
                    .col(pk_auto(Orders::Id))
                    .col(integer(Orders::StatusId))
                    .col(date_time(Orders::CreationDate))
                    .col(date_time(Orders::UpdateDate))
                    .col(boolean(Orders::IsSell))
                    .col(integer(Orders::StrategyId))
                    .col(integer(Orders::BaseAssetId))
                    .col(decimal_len(Orders::BaseAssetAmount, 18, 8))
                    .col(integer(Orders::QuoteAssetId))
                    .col(decimal_len(Orders::QuoteAssetAmount, 18, 8))
                    .col(decimal_len(Orders::PriceEntry, 18, 8))
                    .col(decimal_len(Orders::PriceTarget, 18, 8))
                    .col(decimal_len(Orders::PriceAbort, 18, 8))
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_orders_status")
                    .from(Orders::Table, Orders::StatusId)
                    .to(Status::Table, Status::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_orders_strategies")
                    .from(Orders::Table, Orders::StrategyId)
                    .to(Strategies::Table, Strategies::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_orders_assets-base")
                    .from(Orders::Table, Orders::BaseAssetId)
                    .to(Assets::Table, Assets::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_orders_assets-quote")
                    .from(Orders::Table, Orders::QuoteAssetId)
                    .to(Assets::Table, Assets::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Orders::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Orders {
    Table,
    Id,
    StatusId,
    CreationDate,
    UpdateDate,
    IsSell,
    StrategyId,
    BaseAssetId, // Quantity
    BaseAssetAmount,
    QuoteAssetId, // Price
    QuoteAssetAmount,
    PriceEntry,
    PriceTarget,
    PriceAbort,
}
