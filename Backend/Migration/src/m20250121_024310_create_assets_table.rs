use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Assets::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Assets::Id)
                            .primary_key()
                            .auto_increment()
                            .integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Assets::Name).text().not_null())
                    .col(ColumnDef::new(Assets::Ticker).text().not_null())
                    .col(ColumnDef::new(Assets::Free).decimal_len(18, 8).not_null())
                    .col(ColumnDef::new(Assets::Locked).decimal_len(18, 8).not_null())
                    .to_owned(),
            )
            .await
            .unwrap();

        let insert_assets = Query::insert()
            .into_table(Assets::Table)
            .columns([Assets::Name, Assets::Ticker, Assets::Free, Assets::Locked])
            .values_panic(["Dollar".into(), "USD".into(), 0.into(), 0.into()])
            .values_panic(["Tether".into(), "USDT".into(), 0.into(), 0.into()])
            .values_panic(["Bitcoin".into(), "BTC".into(), 0.into(), 0.into()])
            .values_panic(["Ethereum".into(), "ETH".into(), 0.into(), 0.into()])
            .values_panic(["BNB".into(), "BNB".into(), 0.into(), 0.into()])
            .values_panic(["Cardano".into(), "ADA".into(), 0.into(), 0.into()])
            .values_panic(["USDC".into(), "USDC".into(), 0.into(), 0.into()])
            .to_owned();

        manager.exec_stmt(insert_assets).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Assets::Table).cascade().to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Assets {
    Table,
    Id,
    Name,
    Ticker,
    Free,
    Locked,
}
