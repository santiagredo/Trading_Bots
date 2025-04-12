use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Strategies::Table)
                    .if_not_exists()
                    .col(pk_auto(Strategies::Id))
                    .col(string(Strategies::Name).not_null())
                    .col(boolean(Strategies::IsActive).not_null())
                    .col(boolean(Strategies::CanTrade).not_null())
                    .col(string(Strategies::StreamName).not_null())
                    .col(string(Strategies::Description))
                    .to_owned(),
            )
            .await?;

        let insert_strategies = Query::insert()
            .into_table(Strategies::Table)
            .columns([
                Strategies::Name,
                Strategies::IsActive,
                Strategies::CanTrade,
                Strategies::StreamName,
                Strategies::Description,
            ])
            .values_panic([
                "LTE85%".into(),
                true.into(),
                true.into(),
                "ticker".into(),
                "Buys when price is lesser than or equal to 85%".into(),
            ])
            .values_panic([
                "WAP".into(),
                false.into(),
                false.into(),
                "ticker".into(),
                "Buys when last price deviates x percentage from weighted average price".into(),
            ])
            .values_panic([
                "HGH".into(),
                false.into(),
                false.into(),
                "ticker".into(),
                "Buys when last price deviates x percentage from highest price".into(),
            ])
            .values_panic([
                "OPN".into(),
                false.into(),
                false.into(),
                "ticker".into(),
                "Buys when last price deviates x percentage from open price".into(),
            ])
            .to_owned();

        manager.exec_stmt(insert_strategies).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Strategies::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Strategies {
    Table,
    Id,
    Name,
    IsActive,
    CanTrade,
    StreamName,
    Description,
}
