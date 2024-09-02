use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FillAccessToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FillAccessToken::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FillAccessToken::FormId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_fill_access_form")
                            .from(FillAccessToken::Table, FillAccessToken::FormId)
                            .to(Form::Table, Form::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(FillAccessToken::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(FillAccessToken::ExpiresAt)
                            .timestamp()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FillAccessToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Form {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum FillAccessToken {
    Table,
    Id,
    FormId,
    CreatedAt,
    ExpiresAt,
}
