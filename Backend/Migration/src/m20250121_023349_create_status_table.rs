use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Status::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Status::Id)
                            .primary_key()
                            .auto_increment()
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Status::Name).text().not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        let insert_status = Query::insert()
            .into_table(Status::Table)
            .columns([Status::Name])
            .values_panic(["OPEN".into()])
            .values_panic(["COMPLETED".into()])
            .values_panic(["ABORTED".into()])
            .to_owned();

        manager.exec_stmt(insert_status).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Status::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Status {
    Table,
    Id,
    Name,
}
