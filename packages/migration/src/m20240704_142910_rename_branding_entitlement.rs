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
                    .rename_column(
                        OrganisationFeatureEntitlement::Branding,
                        OrganisationFeatureEntitlement::BrandingCount,
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
                    .rename_column(
                        OrganisationFeatureEntitlement::BrandingCount,
                        OrganisationFeatureEntitlement::Branding,
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum OrganisationFeatureEntitlement {
    Table,
    Branding,
    BrandingCount,
}
