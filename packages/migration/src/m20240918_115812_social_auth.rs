use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(SocialAuthConnection::Table)
                    .col(
                        ColumnDef::new(SocialAuthConnection::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_social_auth_user")
                            .from(SocialAuthConnection::Table, SocialAuthConnection::UserId)
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(SocialAuthConnection::Service)
                            .string()
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(SocialAuthConnection::UserId)
                            .col(SocialAuthConnection::Service),
                    )
                    .col(
                        ColumnDef::new(SocialAuthConnection::Sub)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .col(
                        ColumnDef::new(SocialAuthConnection::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(SocialAuthConnection::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum SocialAuthConnection {
    Table,
    UserId,
    Service,
    Sub,
    CreatedAt,
}
