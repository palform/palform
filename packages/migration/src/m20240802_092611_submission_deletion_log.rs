use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(DeletedSubmission::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(DeletedSubmission::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(DeletedSubmission::DeletedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(DeletedSubmission::FormId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_deleted_submission_form")
                            .from(DeletedSubmission::Table, DeletedSubmission::FormId)
                            .to(Form::Table, Form::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DeletedSubmission::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DeletedSubmission {
    Table,
    Id,
    DeletedAt,
    FormId,
}

#[derive(DeriveIden)]
enum Form {
    Table,
    Id,
}
