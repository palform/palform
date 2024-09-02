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
            .alter_table(
                Table::alter()
                    .table(AdminUser::Table)
                    .add_column(
                        ColumnDef::new(AdminUser::ManualAuthPasswordHash)
                            .string()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(AdminUser::ManualAuthEmailVerified)
                            .boolean()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(AdminUser::OrgAuthOrganisationId)
                            .uuid()
                            .null(),
                    )
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_user_authentication_org")
                            .from_tbl(AdminUser::Table)
                            .from_col(AdminUser::OrgAuthOrganisationId)
                            .to_tbl(Organisation::Table)
                            .to_col(Organisation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .add_column(ColumnDef::new(AdminUser::OrgAuthSub).string().null())
                    .modify_column(
                        ColumnDef::new(AdminUser::Email)
                            .string()
                            .not_null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(AdminUserEmailVerificationPurposeEnum)
                    .values(AdminUserEmailVerificationPurposeVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AdminUserEmailVerification::Table)
                    .col(
                        ColumnDef::new(AdminUserEmailVerification::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AdminUserEmailVerification::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("user_password_reset_user")
                            .from(
                                AdminUserEmailVerification::Table,
                                AdminUserEmailVerification::UserId,
                            )
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(AdminUserEmailVerification::Purpose)
                            .enumeration(
                                AdminUserEmailVerificationPurposeEnum,
                                AdminUserEmailVerificationPurposeVariants::iter(),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AdminUserEmailVerification::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AdminUserEmailVerification::ExpiresAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AdminUserSecondAuthenticationFactor::Table)
                    .col(
                        ColumnDef::new(AdminUserSecondAuthenticationFactor::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AdminUserSecondAuthenticationFactor::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_2fa_user")
                            .from(
                                AdminUserSecondAuthenticationFactor::Table,
                                AdminUserSecondAuthenticationFactor::UserId,
                            )
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(AdminUserSecondAuthenticationFactor::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(AdminUserSecondAuthenticationFactor::TOTPSecret)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrganisationAuthConfig::Table)
                    .col(
                        ColumnDef::new(OrganisationAuthConfig::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OrganisationAuthConfig::OIDCDiscoveryURL)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationAuthConfig::ClientID)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationAuthConfig::ClientSecret)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationAuthConfig::TeamMappingFieldName)
                            .string()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationAuthConfig::RevokeTeamMappings)
                            .boolean()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Organisation::Table)
                    .add_column(ColumnDef::new(Organisation::AuthConfig).uuid().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_org_auth_config")
                            .from_tbl(Organisation::Table)
                            .from_col(Organisation::AuthConfig)
                            .to_tbl(OrganisationAuthConfig::Table)
                            .to_col(OrganisationAuthConfig::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .add_column(
                        ColumnDef::new(Organisation::Subdomain)
                            .string()
                            .null()
                            .unique_key(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrganisationAuthTeamMapping::Table)
                    .col(
                        ColumnDef::new(OrganisationAuthTeamMapping::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(OrganisationAuthTeamMapping::TeamId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_org_team_map_team")
                            .from(
                                OrganisationAuthTeamMapping::Table,
                                OrganisationAuthTeamMapping::TeamId,
                            )
                            .to(Team::Table, Team::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrganisationAuthTeamMapping::Role)
                            .enumeration(
                                OrganisationMemberRoleEnum,
                                OrganisationMemberRoleVariants::iter(),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationAuthTeamMapping::FieldValue)
                            .string()
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
                    .table(AdminUser::Table)
                    .drop_column(AdminUser::ManualAuthPasswordHash)
                    .drop_column(AdminUser::ManualAuthEmailVerified)
                    .drop_column(AdminUser::OrgAuthOrganisationId)
                    .drop_column(AdminUser::OrgAuthSub)
                    .modify_column(ColumnDef::new(AdminUser::Email).string().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(AdminUserEmailVerification::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(
                Type::drop()
                    .name(AdminUserEmailVerificationPurposeEnum)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(AdminUserSecondAuthenticationFactor::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Organisation::Table)
                    .drop_column(Organisation::AuthConfig)
                    .drop_column(Organisation::Subdomain)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(OrganisationAuthConfig::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(
                Table::drop()
                    .table(OrganisationAuthTeamMapping::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Organisation {
    Table,
    Id,
    AuthConfig,
    Subdomain,
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    Id,
    Email,
    ManualAuthPasswordHash,
    ManualAuthEmailVerified,
    OrgAuthOrganisationId,
    OrgAuthSub,
}

#[derive(DeriveIden)]
struct AdminUserEmailVerificationPurposeEnum;
#[derive(DeriveIden, EnumIter)]
enum AdminUserEmailVerificationPurposeVariants {
    NewEmail,
    PasswordReset,
}

#[derive(DeriveIden)]
enum AdminUserEmailVerification {
    Table,
    Id,
    UserId,
    Purpose,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum AdminUserSecondAuthenticationFactor {
    Table,
    Id,
    UserId,
    CreatedAt,
    TOTPSecret,
}

#[derive(DeriveIden)]
enum OrganisationAuthConfig {
    Table,
    Id,
    OIDCDiscoveryURL,
    ClientID,
    ClientSecret,
    TeamMappingFieldName,
    RevokeTeamMappings,
}

#[derive(DeriveIden)]
enum Team {
    Table,
    Id,
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
enum OrganisationAuthTeamMapping {
    Table,
    Id,
    TeamId,
    Role,
    FieldValue,
}
