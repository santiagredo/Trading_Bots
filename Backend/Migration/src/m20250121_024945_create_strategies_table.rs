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
                    .col(ColumnDef::new(Strategies::Description).string().null())
                    .col(ColumnDef::new(Strategies::LastExecution).date_time().null())
                    .col(ColumnDef::new(Strategies::Cooldown).integer().null())
                    .col(ColumnDef::new(Strategies::ErrorLastDate).date_time().null())
                    .col(ColumnDef::new(Strategies::ErrorCooldown).integer().null())
                    .to_owned(),
            )
            .await?;

        let insert_strategies = Query::insert()
            .into_table(Strategies::Table)
            .columns([
                Strategies::Name,
                Strategies::IsActive,
                Strategies::CanTrade,
                Strategies::Description,
                Strategies::Cooldown,
                Strategies::ErrorCooldown,
            ])
            .values_panic([
                "Half value purchase".into(),
                true.into(),
                false.into(),
                "Purchases a given amount if the asset if <=50% from ATH".into(),
                86400.into(),
                120.into(),
            ])
            .values_panic([
                "Near ATH DCA sell".into(),
                true.into(),
                false.into(),
                "Sells a give amount if the asset if >= -5% from ATH".into(),
                86400.into(),
                120.into(),
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
    Description,
    LastExecution,
    Cooldown,
    ErrorLastDate,
    ErrorCooldown,
}
