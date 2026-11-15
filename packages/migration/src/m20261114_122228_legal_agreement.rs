use sea_orm_migration::{
    prelude::*,
    schema::*,
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
                    .as_enum(LegalAgreementTypeEnum)
                    .values(LegalAgreementTypeVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(LegalAgreement::Table)
                    .if_not_exists()
                    .col(big_unsigned(LegalAgreement::Id).primary_key())
                    .col(big_unsigned_null(LegalAgreement::OrganisationId))
                    .col(enumeration(
                        LegalAgreement::AgreementType,
                        LegalAgreementTypeEnum,
                        LegalAgreementTypeVariants::iter(),
                    ))
                    .col(timestamp_with_time_zone_default_now(
                        LegalAgreement::CreatedAt,
                    ))
                    .foreign_key(
                        ForeignKey::create()
                            .from(LegalAgreement::Table, LegalAgreement::OrganisationId)
                            .to(Organisation::Table, Organisation::Id)
                            .on_delete(ForeignKeyAction::SetNull),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(LegalAgreement::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Organisation {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum LegalAgreement {
    Table,
    Id,
    OrganisationId,
    AgreementType,
    CreatedAt,
}

#[derive(DeriveIden)]
struct LegalAgreementTypeEnum;
#[derive(DeriveIden, EnumIter)]
enum LegalAgreementTypeVariants {
    DataProcessing,
}
