use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FormBranding::Table)
                    .add_column(ColumnDef::new(FormBranding::AccentColor).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FormBranding::Table)
                    .drop_column(FormBranding::AccentColor)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FormBranding {
    Table,
    AccentColor,
}
