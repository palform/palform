use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Organisation::Table)
                    .add_column(
                        ColumnDef::new(Organisation::BillingCustomerId)
                            .string()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(Organisation::BillingSubscriptionId)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrganisationFeatureEntitlement::Table)
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::OrganisationId)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_entitlement_org")
                            .from(
                                OrganisationFeatureEntitlement::Table,
                                OrganisationFeatureEntitlement::OrganisationId,
                            )
                            .to(Organisation::Table, Organisation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::UserCount)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::ResponseCount)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::QuestionPerFormCount)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::FormCount)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::TeamCount)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::Export)
                            .not_null()
                            .boolean(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::ImportKeys)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::Branding)
                            .integer()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::Subdomain)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::CryptoDetails)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::Audit)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationFeatureEntitlement::PrioritySupport)
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
                    .table(Organisation::Table)
                    .drop_column(Organisation::BillingCustomerId)
                    .drop_column(Organisation::BillingSubscriptionId)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(OrganisationFeatureEntitlement::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Organisation {
    Table,
    Id,
    BillingCustomerId,
    BillingSubscriptionId,
}

#[derive(DeriveIden)]
enum OrganisationFeatureEntitlement {
    Table,
    OrganisationId,
    UserCount,
    ResponseCount,
    QuestionPerFormCount,
    FormCount,
    TeamCount,
    Export,
    ImportKeys,
    Branding,
    Subdomain,
    CryptoDetails,
    Audit,
    PrioritySupport,
}
