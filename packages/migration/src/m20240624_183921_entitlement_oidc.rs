use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationFeatureEntitlement::Table)
                    .add_column(
                        ColumnDef::new(OrganisationFeatureEntitlement::OIDC)
                            .boolean()
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
                    .table(OrganisationFeatureEntitlement::Table)
                    .drop_column(OrganisationFeatureEntitlement::OIDC)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum OrganisationFeatureEntitlement {
    Table,
    #[allow(clippy::upper_case_acronyms)]
    OIDC,
}
