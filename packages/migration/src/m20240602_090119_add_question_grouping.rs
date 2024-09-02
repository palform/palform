use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(QuestionGroup::Table)
                    .col(
                        ColumnDef::new(QuestionGroup::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(QuestionGroup::Title).string().null())
                    .col(ColumnDef::new(QuestionGroup::Description).string().null())
                    .col(ColumnDef::new(QuestionGroup::Position).integer().not_null())
                    .col(ColumnDef::new(QuestionGroup::FormId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_question_group_form")
                            .from(QuestionGroup::Table, QuestionGroup::FormId)
                            .to(Form::Table, Form::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Question::Table)
                    .add_column(ColumnDef::new(Question::GroupId).uuid().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_question_grouping")
                            .from_tbl(Question::Table)
                            .from_col(Question::GroupId)
                            .to_tbl(QuestionGroup::Table)
                            .to_col(QuestionGroup::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .drop_foreign_key(Alias::new("fk_question_form"))
                    .drop_column(Question::FormId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Question::Table)
                    .drop_foreign_key(Alias::new("fk_question_grouping"))
                    .drop_column(Question::GroupId)
                    .add_column(ColumnDef::new(Question::FormId).uuid().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_question_form")
                            .from_tbl(Question::Table)
                            .from_col(Question::FormId)
                            .to_tbl(Form::Table)
                            .to_col(Form::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(QuestionGroup::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Form {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Question {
    Table,
    GroupId,
    FormId,
}

#[derive(DeriveIden)]
enum QuestionGroup {
    Table,
    Id,
    Title,
    Description,
    FormId,
    Position,
}
