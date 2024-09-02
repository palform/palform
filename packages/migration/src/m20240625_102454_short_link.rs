use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FillAccessToken::Table)
                    .add_column(ColumnDef::new(FillAccessToken::ShortLink).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FillAccessToken::Table)
                    .drop_column(FillAccessToken::ShortLink)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FillAccessToken {
    Table,
    ShortLink,
}
