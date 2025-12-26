use sea_orm_migration::{prelude::*, schema::*};

use crate::m20250121_024945_create_strategies_table::Strategies;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Indicators::Table)
                    .if_not_exists()
                    .col(pk_auto(Indicators::Id))
                    .col(integer(Indicators::StrategyId).not_null())
                    .col(boolean(Indicators::IsActive))
                    .col(string(Indicators::Symbol).not_null())
                    .col(string(Indicators::Nick).not_null())
                    .col(string(Indicators::Direction).not_null())
                    .col(boolean(Indicators::IsPercentage))
                    .col(decimal_len(Indicators::Value, 18, 8))
                    .to_owned(),
            )
            .await
            .unwrap();

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_indicators_strategies")
                    .from(Indicators::Table, Indicators::StrategyId)
                    .to(Strategies::Table, Strategies::Id)
                    .to_owned(),
            )
            .await?;

        let minus_fifty = -50;
        let minus_five = -5;

        let insert_indicators = Query::insert()
            .into_table(Indicators::Table)
            .columns([
                Indicators::StrategyId,
                Indicators::IsActive,
                Indicators::Symbol,
                Indicators::Nick,
                Indicators::Direction,
                Indicators::IsPercentage,
                Indicators::Value,
            ])
            .values_panic([
                1.into(),
                true.into(),
                "BTCUSDT".into(),
                "pfath".into(),
                "lte".into(),
                true.into(),
                minus_fifty.into(),
            ])
            .values_panic([
                2.into(),
                true.into(),
                "BTCUSDT".into(),
                "pfath".into(),
                "gte".into(),
                true.into(),
                minus_five.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert_indicators).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Indicators::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Indicators {
    Table,
    Id,
    StrategyId,
    IsActive,
    Symbol,
    Nick,
    Direction,
    IsPercentage,
    Value,
}
