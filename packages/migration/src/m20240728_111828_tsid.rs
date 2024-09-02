use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

macro_rules! migrate_id {
    ($manager: ident, $table_name: ident) => {
        $manager
            .alter_table(
                Table::alter()
                    .table($table_name::Table)
                    .drop_column($table_name::Id)
                    .add_column(
                        ColumnDef::new($table_name::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await
    };
    ($manager: ident, $table_name: ident, $id_col: expr) => {
        $manager
            .alter_table(
                Table::alter()
                    .table($table_name::Table)
                    .drop_column($id_col)
                    .add_column(
                        ColumnDef::new($id_col)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .to_owned(),
            )
            .await
    };
    ($manager: ident, $table_name: ident, $from_col: expr, $to_tbl: ident, $to_col: expr, $fk_name: expr) => {
        $manager
            .alter_table(
                Table::alter()
                    .table($table_name::Table)
                    .drop_column($from_col)
                    .add_column(ColumnDef::new($from_col).big_unsigned().not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name($fk_name)
                            .from_tbl($table_name::Table)
                            .from_col($from_col)
                            .to_tbl($to_tbl::Table)
                            .to_col($to_col)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    };
}

macro_rules! migrate_id_nullable {
    ($manager: ident, $table_name: ident, $from_col: expr, $to_tbl: ident, $to_col: expr, $fk_name: expr) => {
        $manager
            .alter_table(
                Table::alter()
                    .table($table_name::Table)
                    .drop_column($from_col)
                    .add_column(ColumnDef::new($from_col).big_unsigned().null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name($fk_name)
                            .from_tbl($table_name::Table)
                            .from_col($from_col)
                            .to_tbl($to_tbl::Table)
                            .to_col($to_col)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    };
}

macro_rules! decouple {
    ($manager: ident, $table_name: ident, $fk_name: literal) => {
        $manager
            .alter_table(
                Table::alter()
                    .table($table_name::Table)
                    .drop_foreign_key(Alias::new($fk_name))
                    .to_owned(),
            )
            .await
    };
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        migrate_id!(manager, AdminPublicKey)?;
        migrate_id!(manager, AdminUserEmailVerification)?;
        migrate_id!(manager, AdminUserSecondAuthenticationFactor)?;
        migrate_id!(manager, AdminUserSecondAuthenticationFactorSession)?;
        migrate_id!(manager, AuthToken)?;

        migrate_id!(manager, AuditLogEntry)?;
        manager
            .alter_table(
                Table::alter()
                    .table(AuditLogEntry::Table)
                    .drop_column(AuditLogEntry::TargetResourceId)
                    .add_column(
                        ColumnDef::new(AuditLogEntry::TargetResourceId)
                            .big_integer()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        migrate_id!(manager, Submission)?;
        decouple!(manager, Submission, "fk_submission_fill_token")?;
        migrate_id!(manager, FillAccessToken)?;
        migrate_id_nullable!(
            manager,
            Submission,
            Submission::ForToken,
            FillAccessToken,
            FillAccessToken::Id,
            "fk_submission_fill_token"
        )?;

        migrate_id!(manager, Question)?;
        decouple!(manager, Question, "fk_question_grouping")?;
        migrate_id!(manager, QuestionGroup)?;
        migrate_id!(
            manager,
            Question,
            Question::GroupId,
            QuestionGroup,
            QuestionGroup::Id,
            "fk_question_grouping"
        )?;

        decouple!(manager, FillAccessToken, "fk_fill_access_form")?;
        decouple!(manager, QuestionGroup, "fk_question_group_form")?;
        decouple!(manager, Submission, "fk_submission_form")?;
        migrate_id!(manager, Form)?;
        migrate_id!(
            manager,
            FillAccessToken,
            FillAccessToken::FormId,
            Form,
            Form::Id,
            "fk_fill_access_form"
        )?;
        migrate_id!(
            manager,
            QuestionGroup,
            QuestionGroup::FormId,
            Form,
            Form::Id,
            "fk_question_group_form"
        )?;
        migrate_id!(
            manager,
            Submission,
            Submission::FormId,
            Form,
            Form::Id,
            "fk_submission_form"
        )?;

        decouple!(manager, Form, "fk_form_branding")?;
        migrate_id!(manager, FormBranding)?;
        migrate_id_nullable!(
            manager,
            Form,
            Form::BrandingId,
            FormBranding,
            FormBranding::Id,
            "fk_form_branding"
        )?;

        decouple!(manager, AdminPublicKey, "fk_userkey_org")?;
        decouple!(manager, AdminUser, "fk_user_authentication_org")?;
        decouple!(manager, AuditLogEntry, "fk_audit_log_org")?;
        decouple!(
            manager,
            OrganisationFeatureEntitlement,
            "fk_entitlement_org"
        )?;
        decouple!(
            manager,
            OrganisationInvite,
            "organisation_invite_organisation_id_fkey"
        )?;
        decouple!(manager, OrganisationMembership, "fk_orgmember_organisation")?;
        decouple!(manager, Team, "fk_team_org")?;
        migrate_id!(manager, Organisation)?;
        migrate_id!(
            manager,
            AdminPublicKey,
            AdminPublicKey::OrganisationId,
            Organisation,
            Organisation::Id,
            "fk_admin_public_key_org"
        )?;
        migrate_id_nullable!(
            manager,
            AdminUser,
            AdminUser::OrgAuthOrganisationId,
            Organisation,
            Organisation::Id,
            "fk_user_authentication_org"
        )?;
        migrate_id!(
            manager,
            AuditLogEntry,
            AuditLogEntry::OrganisationId,
            Organisation,
            Organisation::Id,
            "fk_audit_log_org"
        )?;
        migrate_id!(
            manager,
            OrganisationFeatureEntitlement,
            OrganisationFeatureEntitlement::OrganisationId,
            Organisation,
            Organisation::Id,
            "fk_entitlement_org"
        )?;
        migrate_id!(
            manager,
            OrganisationInvite,
            OrganisationInvite::OrganisationId,
            Organisation,
            Organisation::Id,
            "fk_org_invite_org"
        )?;
        migrate_id!(
            manager,
            Team,
            Team::OrganisationId,
            Organisation,
            Organisation::Id,
            "fk_team_org"
        )?;

        decouple!(manager, Organisation, "fk_org_auth_config")?;
        migrate_id!(manager, OrganisationAuthConfig)?;
        migrate_id_nullable!(
            manager,
            Organisation,
            Organisation::AuthConfig,
            OrganisationAuthConfig,
            OrganisationAuthConfig::Id,
            "fk_org_auth_config"
        )?;

        migrate_id!(manager, OrganisationAuthTeamMapping)?;
        migrate_id!(
            manager,
            OrganisationFeatureEntitlement,
            OrganisationFeatureEntitlement::OrganisationId
        )?;
        migrate_id!(manager, OrganisationInvite)?;

        decouple!(manager, AdminPublicKey, "admin_public_key_user_id_fkey")?;
        decouple!(
            manager,
            AdminUserEmailVerification,
            "user_password_reset_user"
        )?;
        decouple!(manager, AdminUserSecondAuthenticationFactor, "fk_2fa_user")?;
        decouple!(
            manager,
            AdminUserSecondAuthenticationFactorSession,
            "fk_2fa_session_user"
        )?;
        decouple!(manager, AuditLogEntry, "fk_audit_log_user")?;
        decouple!(manager, AuthToken, "fk_token_user")?;
        decouple!(manager, OrganisationMembership, "fk_orgmember_user")?;
        decouple!(manager, TeamMembership, "fk_team_member_user")?;
        migrate_id!(manager, AdminUser)?;
        migrate_id!(
            manager,
            AdminPublicKey,
            AdminPublicKey::UserId,
            AdminUser,
            AdminUser::Id,
            "fk_admin_public_key_user"
        )?;
        migrate_id!(
            manager,
            AdminUserSecondAuthenticationFactor,
            AdminUserSecondAuthenticationFactor::UserId,
            AdminUser,
            AdminUser::Id,
            "fk_2fa_user"
        )?;
        migrate_id!(
            manager,
            AdminUserSecondAuthenticationFactorSession,
            AdminUserSecondAuthenticationFactorSession::UserId,
            AdminUser,
            AdminUser::Id,
            "fk_2fa_session_user"
        )?;
        migrate_id!(
            manager,
            AuditLogEntry,
            AuditLogEntry::UserId,
            AdminUser,
            AdminUser::Id,
            "fk_audit_log_user"
        )?;
        migrate_id!(
            manager,
            AuthToken,
            AuthToken::UserId,
            AdminUser,
            AdminUser::Id,
            "fk_token_user"
        )?;
        migrate_id!(
            manager,
            OrganisationMembership,
            OrganisationMembership::UserId,
            AdminUser,
            AdminUser::Id,
            "fk_orgmember_user"
        )?;
        migrate_id!(
            manager,
            TeamMembership,
            TeamMembership::UserId,
            AdminUser,
            AdminUser::Id,
            "fk_team_member_user"
        )?;
        migrate_id!(
            manager,
            AdminUserEmailVerification,
            AdminUserEmailVerification::UserId,
            AdminUser,
            AdminUser::Id,
            "fk_user_email_verification_user"
        )?;

        manager
            .drop_table(
                Table::drop()
                    .table(OrganisationMembership::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(OrganisationMembership::Table)
                    .col(
                        ColumnDef::new(OrganisationMembership::OrganisationId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orgmember_organisation")
                            .from(
                                OrganisationMembership::Table,
                                OrganisationMembership::OrganisationId,
                            )
                            .to(Organisation::Table, Organisation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrganisationMembership::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orgmember_user")
                            .from(
                                OrganisationMembership::Table,
                                OrganisationMembership::UserId,
                            )
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrganisationMembership::IsAdmin)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .primary_key(
                        Index::create()
                            .col(OrganisationMembership::OrganisationId)
                            .col(OrganisationMembership::UserId),
                    )
                    .to_owned(),
            )
            .await?;

        decouple!(manager, FormBranding, "fk_branding_background_image_asset")?;
        decouple!(manager, FormBranding, "fk_branding_logo_asset")?;
        migrate_id!(manager, TeamAsset)?;
        migrate_id_nullable!(
            manager,
            FormBranding,
            FormBranding::BackgroundImageAssetId,
            TeamAsset,
            TeamAsset::Id,
            "fk_branding_background_image_asset"
        )?;
        migrate_id_nullable!(
            manager,
            FormBranding,
            FormBranding::LogoAssetId,
            TeamAsset,
            TeamAsset::Id,
            "fk_branding_logo_asset"
        )?;

        decouple!(manager, Form, "fk_form_team")?;
        decouple!(manager, FormBranding, "fk_branding_team")?;
        decouple!(manager, OrganisationAuthTeamMapping, "fk_org_team_map_team")?;
        decouple!(manager, TeamAsset, "fk_asset_team")?;
        decouple!(manager, TeamMembership, "fk_team_member_team")?;
        migrate_id!(manager, Team)?;
        migrate_id!(manager, Form, Form::TeamId, Team, Team::Id, "fk_form_team")?;
        migrate_id!(
            manager,
            FormBranding,
            FormBranding::TeamId,
            Team,
            Team::Id,
            "fk_branding_team"
        )?;
        migrate_id!(
            manager,
            OrganisationAuthTeamMapping,
            OrganisationAuthTeamMapping::TeamId,
            Team,
            Team::Id,
            "fk_org_team_map_team"
        )?;
        migrate_id!(
            manager,
            TeamAsset,
            TeamAsset::TeamId,
            Team,
            Team::Id,
            "fk_asset_team"
        )?;
        migrate_id!(
            manager,
            TeamMembership,
            TeamMembership::TeamId,
            Team,
            Team::Id,
            "fk_team_member_team"
        )?;

        manager
            .drop_table(Table::drop().table(TeamMembership::Table).to_owned())
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(TeamMembership::Table)
                    .col(
                        ColumnDef::new(TeamMembership::TeamId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_team_member_team")
                            .from(TeamMembership::Table, TeamMembership::TeamId)
                            .to(Team::Table, Team::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(TeamMembership::UserId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_team_member_user")
                            .from(TeamMembership::Table, TeamMembership::UserId)
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(TeamMembership::Role)
                            .enumeration(
                                OrganisationMemberRoleEnum,
                                OrganisationMemberRoleVariants::iter(),
                            )
                            .not_null(),
                    )
                    .primary_key(
                        Index::create()
                            .col(TeamMembership::TeamId)
                            .col(TeamMembership::UserId),
                    )
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        unimplemented!("Cannot reverse the UUID -> TSID transition");
    }
}

#[derive(DeriveIden)]
enum AdminPublicKey {
    Table,
    Id,
    OrganisationId,
    UserId,
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    Id,
    OrgAuthOrganisationId,
}

#[derive(DeriveIden)]
enum Submission {
    Table,
    Id,
    ForToken,
    FormId,
}

#[derive(DeriveIden)]
enum TeamAsset {
    Table,
    Id,
    TeamId,
}

#[derive(DeriveIden)]
enum AdminUserEmailVerification {
    Table,
    Id,
    UserId,
}

#[derive(DeriveIden)]
enum AdminUserSecondAuthenticationFactor {
    Table,
    Id,
    UserId,
}

#[derive(DeriveIden)]
enum AdminUserSecondAuthenticationFactorSession {
    Table,
    Id,
    UserId,
}

#[derive(DeriveIden)]
enum AuditLogEntry {
    Table,
    Id,
    OrganisationId,
    UserId,
    TargetResourceId,
}

#[derive(DeriveIden)]
enum AuthToken {
    Table,
    Id,
    UserId,
}

#[derive(DeriveIden)]
enum FillAccessToken {
    Table,
    Id,
    FormId,
}

#[derive(DeriveIden)]
enum Form {
    Table,
    Id,
    BrandingId,
    TeamId,
}

#[derive(DeriveIden)]
enum FormBranding {
    Table,
    Id,
    BackgroundImageAssetId,
    LogoAssetId,
    TeamId,
}

#[derive(DeriveIden)]
enum Organisation {
    Table,
    Id,
    AuthConfig,
}

#[derive(DeriveIden)]
enum OrganisationAuthConfig {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum OrganisationAuthTeamMapping {
    Table,
    Id,
    TeamId,
}

#[derive(DeriveIden)]
enum OrganisationFeatureEntitlement {
    Table,
    OrganisationId,
}

#[derive(DeriveIden)]
enum OrganisationInvite {
    Table,
    Id,
    OrganisationId,
}

#[derive(DeriveIden)]
enum OrganisationMembership {
    Table,
    OrganisationId,
    UserId,
    IsAdmin,
}

#[derive(DeriveIden)]
enum Question {
    Table,
    Id,
    GroupId,
}

#[derive(DeriveIden)]
enum QuestionGroup {
    Table,
    Id,
    FormId,
}

#[derive(DeriveIden)]
enum Team {
    Table,
    Id,
    OrganisationId,
}

#[derive(DeriveIden)]
enum TeamMembership {
    Table,
    TeamId,
    UserId,
    Role,
}

#[derive(DeriveIden)]
struct OrganisationMemberRoleEnum;
#[derive(DeriveIden, EnumIter)]
enum OrganisationMemberRoleVariants {
    Viewer,
    Editor,
    Admin,
}
