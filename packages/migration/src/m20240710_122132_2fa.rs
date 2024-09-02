use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactor::Table)
                    .add_column(
                        ColumnDef::new(AdminUserSecondAuthenticationFactor::Nickname)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AdminUserSecondAuthenticationFactorSession::Table)
                    .col(
                        ColumnDef::new(AdminUserSecondAuthenticationFactorSession::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AdminUserSecondAuthenticationFactorSession::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_2fa_session_user")
                            .from(
                                AdminUserSecondAuthenticationFactorSession::Table,
                                AdminUserSecondAuthenticationFactorSession::UserId,
                            )
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(AdminUserSecondAuthenticationFactorSession::CreatedAt)
                            .not_null()
                            .timestamp()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AdminUserSecondAuthenticationFactorSession::ExpiresAt)
                            .not_null()
                            .timestamp(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactor::Table)
                    .drop_column(AdminUserSecondAuthenticationFactor::Nickname)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(AdminUserSecondAuthenticationFactorSession::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum AdminUserSecondAuthenticationFactor {
    Table,
    Nickname,
}

#[derive(DeriveIden)]
enum AdminUserSecondAuthenticationFactorSession {
    Table,
    Id,
    UserId,
    CreatedAt,
    ExpiresAt,
}
