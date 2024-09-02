use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Form::Table)
                    .add_column(
                        ColumnDef::new(Form::EnableCaptcha)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationFeatureEntitlement::Table)
                    .add_column(
                        ColumnDef::new(OrganisationFeatureEntitlement::FormCaptcha)
                            .boolean()
                            .default(false)
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
                    .table(Form::Table)
                    .drop_column(Form::EnableCaptcha)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationFeatureEntitlement::Table)
                    .drop_column(OrganisationFeatureEntitlement::FormCaptcha)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Form {
    Table,
    EnableCaptcha,
}

#[derive(DeriveIden)]
enum OrganisationFeatureEntitlement {
    Table,
    FormCaptcha,
}
