use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Question::Table)
                    .add_column(
                        ColumnDef::new(Question::Position)
                            .integer()
                            .default(0)
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Question::Table)
                    .drop_column(Question::Position)
                    .to_owned(),
            )
            .await
    }
}
#[derive(DeriveIden)]
enum Question {
    Table,
    Position,
}
