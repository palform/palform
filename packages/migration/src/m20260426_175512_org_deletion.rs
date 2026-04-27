use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_type(
                Type::create()
                    .as_enum(OrganisationDeletionRequestStatusEnum)
                    .values(OrganisationDeletionRequestStatusVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrganisationDeletionRequest::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganisationDeletionRequest::Id)
                            .big_unsigned()
                            .primary_key(),
                    )
                    // NOT marking org/user ID as a foreign key, since we want to retain them
                    // after deletion.
                    .col(
                        ColumnDef::new(OrganisationDeletionRequest::OrganisationId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationDeletionRequest::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationDeletionRequest::IncludeUser)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationDeletionRequest::Reason)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationDeletionRequest::Status)
                            .enumeration(
                                OrganisationDeletionRequestStatusEnum,
                                OrganisationDeletionRequestStatusVariants::iter(),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationDeletionRequest::CreatedAt)
                            .timestamp_with_time_zone()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(OrganisationDeletionRequest::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(OrganisationDeletionRequestStatusEnum)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum OrganisationDeletionRequest {
    Table,
    Id,
    OrganisationId,
    UserId,
    IncludeUser,
    Reason,
    Status,
    CreatedAt,
}

#[derive(DeriveIden)]
struct OrganisationDeletionRequestStatusEnum;
#[derive(DeriveIden, EnumIter)]
enum OrganisationDeletionRequestStatusVariants {
    GracePeriod,
    Paused,
    Deleted,
    Cancelled,
}
