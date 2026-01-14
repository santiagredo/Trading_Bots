use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Integrations::Table)
                    .if_not_exists()
                    .col(pk_auto(Integrations::Id))
                    .col(string(Integrations::Name))
                    .col(string(Integrations::Code))
                    .col(boolean(Integrations::IsEnabled).default(true))
                    .col(timestamp(Integrations::CreationDate).default(Expr::current_timestamp()))
                    .col(timestamp(Integrations::LastUpdateDate).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(Integrations::Table)
                    .columns([
                        Integrations::Name,
                        Integrations::Code,
                        Integrations::IsEnabled,
                    ])
                    .values_panic(["Binance".into(), "BINANCE".into(), true.into()])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(Integrations::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Integrations {
    Table,
    Id,
    Name,
    Code,
    IsEnabled,
    CreationDate,
    LastUpdateDate,
}
