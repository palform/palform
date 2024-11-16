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
                    .add_column(
                        ColumnDef::new(FormBranding::BackgroundColor)
                            .string()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(FormBranding::BackgroundColorAccent)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FormBranding::Table)
                    .drop_column(FormBranding::BackgroundColor)
                    .drop_column(FormBranding::BackgroundColorAccent)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FormBranding {
    Table,
    BackgroundColor,
    BackgroundColorAccent,
}
