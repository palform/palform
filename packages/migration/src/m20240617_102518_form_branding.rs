use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FormBranding::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FormBranding::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FormBranding::TeamId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_branding_team")
                            .from(FormBranding::Table, FormBranding::TeamId)
                            .to(Team::Table, Team::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(FormBranding::Name).string().not_null())
                    .col(ColumnDef::new(FormBranding::GoogleFont).string().not_null())
                    .col(
                        ColumnDef::new(FormBranding::PrimaryColor)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Form::Table)
                    .add_column(ColumnDef::new(Form::BrandingId).uuid().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_form_branding")
                            .from_tbl(Form::Table)
                            .from_col(Form::BrandingId)
                            .to_tbl(FormBranding::Table)
                            .to_col(FormBranding::Id)
                            .on_delete(ForeignKeyAction::SetNull),
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
                    .drop_column(Form::BrandingId)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(FormBranding::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Form {
    Table,
    BrandingId,
}

#[derive(DeriveIden)]
enum FormBranding {
    Table,
    Id,
    TeamId,
    Name,
    GoogleFont,
    PrimaryColor,
}

#[derive(DeriveIden)]
enum Team {
    Table,
    Id,
}
