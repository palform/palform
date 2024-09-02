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
                    .drop_column(FormBranding::HasLogo)
                    .drop_column(FormBranding::HasBackgroundImage)
                    .add_column(ColumnDef::new(FormBranding::LogoAssetId).uuid().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_branding_logo_asset")
                            .from_tbl(FormBranding::Table)
                            .from_col(FormBranding::LogoAssetId)
                            .to_tbl(TeamAsset::Table)
                            .to_col(TeamAsset::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .add_column(
                        ColumnDef::new(FormBranding::BackgroundImageAssetId)
                            .uuid()
                            .null(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_branding_background_image_asset")
                            .from_tbl(FormBranding::Table)
                            .from_col(FormBranding::BackgroundImageAssetId)
                            .to_tbl(TeamAsset::Table)
                            .to_col(TeamAsset::Id)
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
                    .table(FormBranding::Table)
                    .drop_column(FormBranding::LogoAssetId)
                    .drop_column(FormBranding::BackgroundImageAssetId)
                    .add_column(
                        ColumnDef::new(FormBranding::HasLogo)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .add_column(
                        ColumnDef::new(FormBranding::HasBackgroundImage)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FormBranding {
    Table,
    HasLogo,
    HasBackgroundImage,
    LogoAssetId,
    BackgroundImageAssetId,
}

#[derive(DeriveIden)]
enum TeamAsset {
    Table,
    Id,
}
