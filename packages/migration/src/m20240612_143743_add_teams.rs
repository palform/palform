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
                    .table(Team::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Team::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Team::Name).string().not_null())
                    .col(ColumnDef::new(Team::OrganisationId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Team::Table, Team::OrganisationId)
                            .to(Organisation::Table, Organisation::Id)
                            .name("fk_team_org")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(TeamMembership::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(TeamMembership::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(TeamMembership::Table, TeamMembership::UserId)
                            .to(AdminUser::Table, AdminUser::Id)
                            .name("fk_team_member_user")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(TeamMembership::TeamId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(TeamMembership::Table, TeamMembership::TeamId)
                            .to(Team::Table, Team::Id)
                            .name("fk_team_member_team")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(TeamMembership::UserId)
                            .col(TeamMembership::TeamId),
                    )
                    .col(
                        ColumnDef::new(TeamMembership::Role)
                            .enumeration(
                                OrganisationMemberRoleEnum,
                                OrganisationMemberRoleVariants::iter(),
                            )
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Form::Table)
                    .drop_column(Form::OwnerOrganisationId)
                    .add_column(ColumnDef::new(Form::TeamId).uuid().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .from_tbl(Form::Table)
                            .from_col(Form::TeamId)
                            .to_tbl(Team::Table)
                            .to_col(Team::Id)
                            .name("fk_form_team")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationMembership::Table)
                    .drop_column(OrganisationMembership::Role)
                    .add_column(
                        ColumnDef::new(OrganisationMembership::IsAdmin)
                            .not_null()
                            .boolean(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationInvite::Table)
                    .drop_column(OrganisationInvite::Role)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Form::Table)
                    .drop_column(Form::TeamId)
                    .add_column(ColumnDef::new(Form::OwnerOrganisationId).uuid().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .from_tbl(Form::Table)
                            .from_col(Form::OwnerOrganisationId)
                            .to_tbl(Organisation::Table)
                            .to_col(Organisation::Id)
                            .name("fk_form_organisation")
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(TeamMembership::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Team::Table).to_owned())
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationMembership::Table)
                    .add_column(
                        ColumnDef::new(OrganisationMembership::Role)
                            .enumeration(
                                OrganisationMemberRoleEnum,
                                OrganisationMemberRoleVariants::iter(),
                            )
                            .not_null(),
                    )
                    .drop_column(OrganisationMembership::IsAdmin)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationInvite::Table)
                    .add_column(
                        ColumnDef::new(OrganisationInvite::Role)
                            .not_null()
                            .enumeration(
                                OrganisationMemberRoleEnum,
                                OrganisationMemberRoleVariants::iter(),
                            ),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Organisation {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Team {
    Table,
    Id,
    Name,
    OrganisationId,
}

#[derive(DeriveIden)]
enum TeamMembership {
    Table,
    UserId,
    TeamId,
    Role,
}

#[derive(DeriveIden)]
enum Form {
    Table,
    OwnerOrganisationId,
    TeamId,
}

#[derive(DeriveIden)]
struct OrganisationMemberRoleEnum;
#[derive(DeriveIden, EnumIter)]
enum OrganisationMemberRoleVariants {
    Viewer,
    Editor,
    Admin,
}

#[derive(DeriveIden)]
enum OrganisationMembership {
    Table,
    Role,
    IsAdmin,
}

#[derive(DeriveIden)]
enum OrganisationInvite {
    Table,
    Role,
}
