use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

const FOREIGN_KEYS: &[(&str, &str, &str, &str, &str)] = &[
    // (name, from table, from column, to table, to column)
    (
        "fk_form_branding",
        "form",
        "branding_id",
        "form_branding",
        "id",
    ),
    (
        "fk_org_auth_config",
        "organisation",
        "auth_config",
        "organisation_auth_config",
        "id",
    ),
    (
        "fk_branding_logo_asset",
        "form_branding",
        "logo_asset_id",
        "team_asset",
        "id",
    ),
    (
        "fk_branding_background_image_asset",
        "form_branding",
        "background_image_asset_id",
        "team_asset",
        "id",
    ),
    (
        "fk_submission_fill_token",
        "submission",
        "for_token",
        "fill_access_token",
        "id",
    ),
];

async fn set_on_delete(manager: &SchemaManager<'_>, action: ForeignKeyAction) -> Result<(), DbErr> {
    for &(name, from_tbl, from_col, to_tbl, to_col) in FOREIGN_KEYS {
        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new(from_tbl))
                    .drop_foreign_key(Alias::new(name))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Alias::new(from_tbl))
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name(name)
                            .from_tbl(Alias::new(from_tbl))
                            .from_col(Alias::new(from_col))
                            .to_tbl(Alias::new(to_tbl))
                            .to_col(Alias::new(to_col))
                            .on_delete(action.clone()),
                    )
                    .to_owned(),
            )
            .await?;
    }

    Ok(())
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        set_on_delete(manager, ForeignKeyAction::SetNull).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        set_on_delete(manager, ForeignKeyAction::Cascade).await
    }
}
