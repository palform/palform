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
                        ColumnDef::new(AdminPublicKey::CertFingerprint)
                            .string()
                            .not_null()
                            .unique_key(),
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
                    .drop_column(AdminPublicKey::CertFingerprint)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AdminPublicKey {
    Table,
    CertFingerprint,
}
