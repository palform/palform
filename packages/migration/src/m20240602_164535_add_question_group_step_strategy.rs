use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(QuestionGroup::Table)
                    .add_column(
                        ColumnDef::new(QuestionGroup::StepStrategy)
                            .json_binary()
                            .default(Expr::value("\"NextPosition\""))
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
                    .table(QuestionGroup::Table)
                    .drop_column(QuestionGroup::StepStrategy)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum QuestionGroup {
    Table,
    StepStrategy,
}
