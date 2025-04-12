use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250121_024945_create_strategies_table::Strategies,
    m20250130_013341_create_pair_assets_table::PairAssets,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(StrategiesPairAssets::Table)
                    .if_not_exists()
                    .col(pk_auto(StrategiesPairAssets::Id))
                    .col(integer(StrategiesPairAssets::StrategyId))
                    .col(integer(StrategiesPairAssets::PairAssetId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_strategies-pair-assets_strategies")
                    .from(
                        StrategiesPairAssets::Table,
                        StrategiesPairAssets::StrategyId,
                    )
                    .to(Strategies::Table, Strategies::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_strategies-pair-assets_pair-assets")
                    .from(
                        StrategiesPairAssets::Table,
                        StrategiesPairAssets::PairAssetId,
                    )
                    .to(PairAssets::Table, PairAssets::Id)
                    .to_owned(),
            )
            .await?;

        let insert_strategy_pair_asset = Query::insert()
            .into_table(StrategiesPairAssets::Table)
            .columns([
                StrategiesPairAssets::StrategyId,
                StrategiesPairAssets::PairAssetId,
            ])
            .values_panic([1.into(), 1.into()])
            .values_panic([2.into(), 1.into()])
            .values_panic([3.into(), 1.into()])
            .values_panic([4.into(), 1.into()])
            .to_owned();

        manager.exec_stmt(insert_strategy_pair_asset).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(StrategiesPairAssets::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum StrategiesPairAssets {
    Table,
    Id,
    StrategyId,
    PairAssetId,
}
