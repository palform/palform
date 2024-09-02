use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(OrganisationInvite::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(OrganisationInvite::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OrganisationInvite::OrganisationId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(
                                OrganisationInvite::Table,
                                OrganisationInvite::OrganisationId,
                            )
                            .to(Organisation::Table, Organisation::Id),
                    )
                    .col(
                        ColumnDef::new(OrganisationInvite::Role)
                            .enumeration(
                                OrganisationMemberRoleEnum,
                                OrganisationMemberRoleVariants::iter(),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationInvite::SingleUse)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationInvite::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(OrganisationInvite::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(OrganisationInvite::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Organisation {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum OrganisationInvite {
    Table,
    Id,
    OrganisationId,
    Role,
    SingleUse,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
struct OrganisationMemberRoleEnum;
#[derive(DeriveIden, EnumIter)]
enum OrganisationMemberRoleVariants {
    Viewer,
    Editor,
    Admin,
}
