use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(FormBrandingBorderRoundingEnum)
                    .values(FormBrandingBorderRoundingVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(FormBrandingSpacingEnum)
                    .values(FormBrandingSpacingVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(FormBrandingFontSizeEnum)
                    .values(FormBrandingFontSizeVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FormBranding::Table)
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
                    .add_column(
                        ColumnDef::new(FormBranding::BorderRounding)
                            .enumeration(
                                FormBrandingBorderRoundingEnum,
                                FormBrandingBorderRoundingVariants::iter(),
                            )
                            .not_null(),
                    )
                    .add_column(
                        ColumnDef::new(FormBranding::FontSize)
                            .enumeration(
                                FormBrandingFontSizeEnum,
                                FormBrandingFontSizeVariants::iter(),
                            )
                            .not_null(),
                    )
                    .add_column(
                        ColumnDef::new(FormBranding::Spacing)
                            .enumeration(
                                FormBrandingSpacingEnum,
                                FormBrandingSpacingVariants::iter(),
                            )
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
                    .table(FormBranding::Table)
                    .drop_column(FormBranding::HasLogo)
                    .drop_column(FormBranding::HasBackgroundImage)
                    .drop_column(FormBranding::BorderRounding)
                    .drop_column(FormBranding::FontSize)
                    .drop_column(FormBranding::Spacing)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(FormBrandingBorderRoundingEnum).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(FormBrandingSpacingEnum).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(FormBrandingFontSizeEnum).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
struct FormBrandingBorderRoundingEnum;
#[derive(DeriveIden, EnumIter)]
enum FormBrandingBorderRoundingVariants {
    None,
    Small,
    Medium,
    Large,
}

#[derive(DeriveIden)]
struct FormBrandingSpacingEnum;
#[derive(DeriveIden, EnumIter)]
enum FormBrandingSpacingVariants {
    Tight,
    Normal,
    Comfy,
}

#[derive(DeriveIden)]
struct FormBrandingFontSizeEnum;
#[derive(DeriveIden, EnumIter)]
enum FormBrandingFontSizeVariants {
    Tiny,
    Small,
    Regular,
    Large,
    VeryLarge,
}

#[derive(DeriveIden)]
enum FormBranding {
    Table,
    HasLogo,
    HasBackgroundImage,
    BorderRounding,
    FontSize,
    Spacing,
}
