use sea_orm_migration::{prelude::*, schema::*};

use crate::{
    m20250121_024945_create_strategies_table::Strategies,
    m20250130_013341_create_pairs_table::Pairs,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Actions::Table)
                    .if_not_exists()
                    .col(pk_auto(Actions::Id))
                    .col(integer(Actions::StrategyId))
                    .col(boolean(Actions::IsActive))
                    .col(boolean(Actions::IsSell))
                    .col(boolean(Actions::IsQuoteAsset))
                    .col(boolean(Actions::IsPercentage))
                    .col(decimal_len(Actions::Value, 18, 8))
                    .col(integer(Actions::PairId))
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_actions_strategies")
                    .from(Actions::Table, Actions::StrategyId)
                    .to(Strategies::Table, Strategies::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_actions_pairs")
                    .from(Actions::Table, Actions::PairId)
                    .to(Pairs::Table, Pairs::Id)
                    .to_owned(),
            )
            .await?;

        let insert_actions = Query::insert()
            .into_table(Actions::Table)
            .columns([
                Actions::StrategyId,
                Actions::IsActive,
                Actions::IsSell,
                Actions::IsQuoteAsset,
                Actions::IsPercentage,
                Actions::Value,
                Actions::PairId,
            ])
            .values_panic([
                1.into(),
                true.into(),
                false.into(),
                true.into(),
                false.into(),
                10.into(),
                1.into(),
            ])
            .values_panic([
                2.into(),
                true.into(),
                true.into(),
                true.into(),
                false.into(),
                10.into(),
                1.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert_actions).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Actions::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Actions {
    Table,
    Id,
    StrategyId,
    IsActive,
    IsSell,
    IsQuoteAsset,
    IsPercentage,
    Value,
    PairId,
}
