use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Tasks::Table)
                    .if_not_exists()
                    .col(pk_auto(Tasks::Id))
                    .col(string(Tasks::Nick))
                    .col(string(Tasks::Description))
                    .col(boolean(Tasks::IsActive))
                    .col(big_integer(Tasks::Cooldown))
                    .col(big_integer(Tasks::Delay))
                    .to_owned(),
            )
            .await
            .unwrap();

        let insert_tasks = Query::insert()
            .into_table(Tasks::Table)
            .columns([
                Tasks::Nick,
                Tasks::Description,
                Tasks::IsActive,
                Tasks::Cooldown,
                Tasks::Delay,
            ])
            .values_panic([
                "BNUAB".into(),
                "Gets and saves Binance's account balances locally".into(),
                true.into(),
                3600.into(),
                0.into(),
            ])
            .values_panic([
                "BNUEI".into(),
                "Gets and saves Binance's exchange information locally".into(),
                true.into(),
                86400.into(),
                0.into(),
            ])
            .values_panic([
                "CPUPS".into(),
                "Gets and saves Coin Paprika's pairs statistics locally".into(),
                true.into(),
                3600.into(),
                0.into(),
            ])
            .values_panic([
                "CMPER".into(),
                "Gets and saves internal metrics statistics locally".into(),
                true.into(),
                3600.into(),
                0.into(),
            ])
            .to_owned();

        manager.exec_stmt(insert_tasks).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Tasks::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Tasks {
    Table,
    Id,
    Nick,
    Description,
    IsActive,
    Cooldown,
    Delay,
}
