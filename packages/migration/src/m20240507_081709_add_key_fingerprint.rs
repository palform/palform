use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminPublicKey::Table)
                    .add_column(
                        ColumnDef::new(AdminPublicKey::KeyFingerprint)
                            .string_len(40)
                            .unique_key()
                            .not_null(),
                    )
                    .add_column(
                        ColumnDef::new(AdminPublicKey::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .add_column(
                        ColumnDef::new(AdminPublicKey::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .add_column(
                        ColumnDef::new(AdminPublicKey::OrganisationId)
                            .uuid()
                            .not_null(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_userkey_org")
                            .from_tbl(AdminPublicKey::Table)
                            .from_col(AdminPublicKey::OrganisationId)
                            .to_tbl(Organisation::Table)
                            .to_col(Organisation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminPublicKey::Table)
                    .drop_column(AdminPublicKey::KeyFingerprint)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AdminPublicKey {
    Table,
    KeyFingerprint,
    OrganisationId,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum Organisation {
    Table,
    Id,
}
