use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactorSession::Table)
                    .add_column(
                        ColumnDef::new(AdminUserSecondAuthenticationFactorSession::WebauthnSecret)
                            .json_binary()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(AdminUserSecondAuthenticationFactorSession::IsCreate)
                            .boolean()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactor::Table)
                    .add_column(
                        ColumnDef::new(AdminUserSecondAuthenticationFactor::WebauthnPublicKey)
                            .json_binary()
                            .null(),
                    )
                    .modify_column(
                        ColumnDef::new(AdminUserSecondAuthenticationFactor::TotpSecret)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactorSession::Table)
                    .drop_column(AdminUserSecondAuthenticationFactorSession::WebauthnSecret)
                    .drop_column(AdminUserSecondAuthenticationFactorSession::IsCreate)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactor::Table)
                    .drop_column(AdminUserSecondAuthenticationFactor::WebauthnPublicKey)
                    .modify_column(
                        ColumnDef::new(AdminUserSecondAuthenticationFactor::TotpSecret)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AdminUserSecondAuthenticationFactor {
    Table,
    TotpSecret,
    WebauthnPublicKey,
}

#[derive(DeriveIden)]
enum AdminUserSecondAuthenticationFactorSession {
    Table,
    WebauthnSecret,
    IsCreate,
}
