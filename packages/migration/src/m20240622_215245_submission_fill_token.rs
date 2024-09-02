use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Submission::Table)
                    .add_column(ColumnDef::new(Submission::ForToken).uuid().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_submission_fill_token")
                            .from_tbl(Submission::Table)
                            .from_col(Submission::ForToken)
                            .to_tbl(FillAccessToken::Table)
                            .to_col(FillAccessToken::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(FillAccessToken::Table)
                    .add_column(
                        ColumnDef::new(FillAccessToken::Nickname)
                            .string()
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
                    .table(Submission::Table)
                    .drop_column(Submission::ForToken)
                    .to_owned(),
            )
            .await?;
        manager
            .alter_table(
                Table::alter()
                    .table(FillAccessToken::Table)
                    .drop_column(FillAccessToken::Nickname)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Submission {
    Table,
    ForToken,
}

#[derive(DeriveIden)]
enum FillAccessToken {
    Table,
    Id,
    Nickname,
}
