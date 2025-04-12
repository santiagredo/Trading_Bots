use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RecordTypes::Table)
                    .if_not_exists()
                    .col(pk_auto(RecordTypes::Id))
                    .col(string(RecordTypes::Name))
                    .to_owned(),
            )
            .await?;

        let insert_types = Query::insert()
            .into_table(RecordTypes::Table)
            .columns([RecordTypes::Name])
            .values_panic(["BUY".into()])
            .values_panic(["SELL".into()])
            .values_panic(["ADJUST".into()])
            .to_owned();

        manager.exec_stmt(insert_types).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(RecordTypes::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RecordTypes {
    Table,
    Id,
    Name,
}
