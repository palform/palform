use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FormBranding::Table)
                    .add_column(
                        ColumnDef::new(FormBranding::IncludePalformAttribution)
                            .boolean()
                            .not_null(),
                    )
                    .add_column(ColumnDef::new(FormBranding::TermsLink).string().null())
                    .add_column(ColumnDef::new(FormBranding::PrivacyLink).string().null())
                    .add_column(
                        ColumnDef::new(FormBranding::ExtraFooterMessage)
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
                    .table(FormBranding::Table)
                    .drop_column(FormBranding::IncludePalformAttribution)
                    .drop_column(FormBranding::TermsLink)
                    .drop_column(FormBranding::PrivacyLink)
                    .drop_column(FormBranding::ExtraFooterMessage)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FormBranding {
    Table,
    IncludePalformAttribution,
    TermsLink,
    PrivacyLink,
    ExtraFooterMessage,
}
