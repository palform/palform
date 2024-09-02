use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(AdminPublicKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminPublicKey::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AdminPublicKey::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(AdminPublicKey::Table, AdminPublicKey::UserId)
                            .to(AdminUser::Table, AdminUser::Id),
                    )
                    .col(
                        ColumnDef::new(AdminPublicKey::PublicKey)
                            .binary()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AdminPublicKey::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum AdminPublicKey {
    Table,
    Id,
    UserId,
    PublicKey,
}
