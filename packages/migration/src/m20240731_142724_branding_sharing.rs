use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FormBrandingTeamAccess::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FormBrandingTeamAccess::FormBrandingId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_form_branding_access_form_branding")
                            .from(
                                FormBrandingTeamAccess::Table,
                                FormBrandingTeamAccess::FormBrandingId,
                            )
                            .to(FormBranding::Table, FormBranding::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(FormBrandingTeamAccess::TeamId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_form_branding_access_team")
                            .from(
                                FormBrandingTeamAccess::Table,
                                FormBrandingTeamAccess::TeamId,
                            )
                            .to(Team::Table, Team::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(FormBrandingTeamAccess::FormBrandingId)
                            .col(FormBrandingTeamAccess::TeamId),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FormBranding::Table)
                    .drop_column(FormBranding::TeamId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(FormBranding::Table)
                    .add_column(
                        ColumnDef::new(FormBranding::TeamId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_form_branding_team")
                            .from_tbl(FormBranding::Table)
                            .from_col(FormBranding::TeamId)
                            .to_tbl(Team::Table)
                            .to_col(Team::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(FormBrandingTeamAccess::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FormBrandingTeamAccess {
    Table,
    FormBrandingId,
    TeamId,
}

#[derive(DeriveIden)]
enum FormBranding {
    Table,
    Id,
    TeamId,
}

#[derive(DeriveIden)]
enum Team {
    Table,
    Id,
}
