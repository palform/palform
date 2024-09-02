use extension::postgres::Type;
use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(FormBrandingBorderIntensityEnum)
                    .values(FormBrandingBorderIntensityVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FormBranding::Table)
                    .add_column(
                        ColumnDef::new(FormBranding::BorderIntensity)
                            .enumeration(
                                FormBrandingBorderIntensityEnum,
                                FormBrandingBorderIntensityVariants::iter(),
                            )
                            .not_null()
                            .default(Expr::value("medium")),
                    )
                    .add_column(
                        ColumnDef::new(FormBranding::BorderShadowIntensity)
                            .enumeration(
                                FormBrandingBorderIntensityEnum,
                                FormBrandingBorderIntensityVariants::iter(),
                            )
                            .not_null()
                            .default(Expr::value("medium")),
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
                    .drop_column(FormBranding::BorderIntensity)
                    .drop_column(FormBranding::BorderShadowIntensity)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FormBranding {
    Table,
    BorderIntensity,
    BorderShadowIntensity,
}

#[derive(DeriveIden)]
struct FormBrandingBorderIntensityEnum;
#[derive(DeriveIden, EnumIter)]
enum FormBrandingBorderIntensityVariants {
    Off,
    Low,
    Medium,
    High,
}
