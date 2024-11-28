//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.0

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    schemars :: JsonSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "admin_user_email_verification_purpose_enum"
)]
pub enum AdminUserEmailVerificationPurposeEnum {
    #[sea_orm(string_value = "new_email")]
    NewEmail,
    #[sea_orm(string_value = "password_reset")]
    PasswordReset,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    schemars :: JsonSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "audit_log_target_resource_enum"
)]
pub enum AuditLogTargetResourceEnum {
    #[sea_orm(string_value = "auth_session")]
    AuthSession,
    #[sea_orm(string_value = "branding")]
    Branding,
    #[sea_orm(string_value = "form")]
    Form,
    #[sea_orm(string_value = "organisation")]
    Organisation,
    #[sea_orm(string_value = "organisation_auth_config")]
    OrganisationAuthConfig,
    #[sea_orm(string_value = "organisation_member")]
    OrganisationMember,
    #[sea_orm(string_value = "organisation_subdomain")]
    OrganisationSubdomain,
    #[sea_orm(string_value = "team")]
    Team,
    #[sea_orm(string_value = "team_member")]
    TeamMember,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    schemars :: JsonSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "audit_log_verb_enum"
)]
pub enum AuditLogVerbEnum {
    #[sea_orm(string_value = "create")]
    Create,
    #[sea_orm(string_value = "delete")]
    Delete,
    #[sea_orm(string_value = "read")]
    Read,
    #[sea_orm(string_value = "update")]
    Update,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    schemars :: JsonSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "form_branding_border_intensity_enum"
)]
pub enum FormBrandingBorderIntensityEnum {
    #[sea_orm(string_value = "high")]
    High,
    #[sea_orm(string_value = "low")]
    Low,
    #[sea_orm(string_value = "medium")]
    Medium,
    #[sea_orm(string_value = "off")]
    Off,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    schemars :: JsonSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "form_branding_border_rounding_enum"
)]
pub enum FormBrandingBorderRoundingEnum {
    #[sea_orm(string_value = "large")]
    Large,
    #[sea_orm(string_value = "medium")]
    Medium,
    #[sea_orm(string_value = "none")]
    None,
    #[sea_orm(string_value = "small")]
    Small,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    schemars :: JsonSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "form_branding_font_size_enum"
)]
pub enum FormBrandingFontSizeEnum {
    #[sea_orm(string_value = "large")]
    Large,
    #[sea_orm(string_value = "regular")]
    Regular,
    #[sea_orm(string_value = "small")]
    Small,
    #[sea_orm(string_value = "tiny")]
    Tiny,
    #[sea_orm(string_value = "very_large")]
    VeryLarge,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    schemars :: JsonSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "form_branding_spacing_enum"
)]
pub enum FormBrandingSpacingEnum {
    #[sea_orm(string_value = "comfy")]
    Comfy,
    #[sea_orm(string_value = "normal")]
    Normal,
    #[sea_orm(string_value = "tight")]
    Tight,
}
#[derive(
    Debug,
    Clone,
    PartialEq,
    Eq,
    EnumIter,
    DeriveActiveEnum,
    Serialize,
    Deserialize,
    schemars :: JsonSchema,
)]
#[sea_orm(
    rs_type = "String",
    db_type = "Enum",
    enum_name = "organisation_member_role_enum"
)]
pub enum OrganisationMemberRoleEnum {
    #[sea_orm(string_value = "admin")]
    Admin,
    #[sea_orm(string_value = "editor")]
    Editor,
    #[sea_orm(string_value = "viewer")]
    Viewer,
}
