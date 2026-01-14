use sea_orm_migration::{prelude::*, schema::*};

use crate::m20260114_021737_create_integrations_table::Integrations;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(IntegrationSettings::Table)
                    .if_not_exists()
                    .col(pk_auto(IntegrationSettings::Id))
                    .col(integer(IntegrationSettings::IntegrationId))
                    .col(string(IntegrationSettings::Name))
                    .col(string(IntegrationSettings::Nick))
                    .col(string(IntegrationSettings::Value))
                    .col(
                        timestamp(IntegrationSettings::CreationDate)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(IntegrationSettings::LastUpdateDate)
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-integration-settings-integration")
                            .from(
                                IntegrationSettings::Table,
                                IntegrationSettings::IntegrationId,
                            )
                            .to(Integrations::Table, Integrations::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(IntegrationSettings::Table)
                    .columns([
                        IntegrationSettings::IntegrationId,
                        IntegrationSettings::Name,
                        IntegrationSettings::Nick,
                        IntegrationSettings::Value,
                    ])
                    .values_panic([
                        1.into(),
                        "Api Key".into(), // name
                        "api_key".into(), // nick
                        "".into(),        // value
                    ])
                    .to_owned(),
            )
            .await?;

        manager
            .exec_stmt(
                Query::insert()
                    .into_table(IntegrationSettings::Table)
                    .columns([
                        IntegrationSettings::IntegrationId,
                        IntegrationSettings::Name,
                        IntegrationSettings::Nick,
                        IntegrationSettings::Value,
                    ])
                    .values_panic([
                        1.into(),
                        "Secret Pass".into(), // name
                        "secret_pass".into(), // nick
                        "".into(),            // value
                    ])
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(IntegrationSettings::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum IntegrationSettings {
    Table,
    Id,
    IntegrationId,
    Name,
    Nick,
    Value,
    CreationDate,
    LastUpdateDate,
}
