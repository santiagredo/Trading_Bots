use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IntegrationLog::Table)
                    .if_not_exists()
                    .col(pk_auto(IntegrationLog::Id))
                    .col(date_time(IntegrationLog::CreationDate))
                    .col(string(IntegrationLog::IntegrationName))
                    .col(string(IntegrationLog::FunctionName))
                    .col(string(IntegrationLog::Url))
                    .col(string(IntegrationLog::Request))
                    .col(string(IntegrationLog::Response))
                    .col(integer(IntegrationLog::StatusCode))
                    .col(string(IntegrationLog::ErrorMessage))
                    .col(integer(IntegrationLog::ExecutionTimeMs))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(IntegrationLog::Table)
                    .cascade()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum IntegrationLog {
    Table,
    Id,
    CreationDate,
    IntegrationName,
    FunctionName,
    Url,
    Request,
    Response,
    StatusCode,
    ErrorMessage,
    ExecutionTimeMs,
}
