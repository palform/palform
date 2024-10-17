use std::fmt::Display;

use palform_entities::organisation_feature_entitlement;
use palform_tsid::{resources::IDForm, tsid::PalformDatabaseID};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::{FromQueryResult, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub enum APIEntitlementRequest {
    UserCount,
    ResponseCount,
    QuestionPerFormCount(PalformDatabaseID<IDForm>),
    FormCount,
    TeamCount,
    BrandingCount,
    Export,
    ImportKeys,
    Subdomain,
    CryptoDetails,
    Audit,
    PrioritySupport,
    #[allow(clippy::upper_case_acronyms)]
    OIDC,
    SubmissionAutoDelete,
    FormCaptcha,
}

impl Display for APIEntitlementRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.to_owned() {
            Self::UserCount => write!(f, "Too many users"),
            Self::ResponseCount => write!(f, "Too many responses/month"),
            Self::QuestionPerFormCount(_) => write!(f, "Too many questions in form"),
            Self::FormCount => write!(f, "Too many forms"),
            Self::TeamCount => write!(f, "Too many teams"),
            Self::BrandingCount => write!(f, "Too many branding schemes"),
            Self::Export => write!(f, "Exporting data is not allowed"),
            Self::ImportKeys => write!(f, "Importing keys is not allowed"),
            Self::Subdomain => write!(f, "Subdomains are not allowed"),
            Self::CryptoDetails => write!(f, "Viewing encryption details is not allowed"),
            Self::Audit => write!(f, "Audits are not allowed"),
            Self::PrioritySupport => write!(f, "Priority support is not available"),
            Self::OIDC => write!(f, "OpenID Connect is not available"),
            Self::SubmissionAutoDelete => write!(f, "Auto-deleting submissions is not available"),
            Self::FormCaptcha => write!(f, "Captcha-protected forms are not available"),
        }
    }
}

#[derive(Deserialize, Serialize, FromQueryResult, JsonSchema)]
pub struct APIEntitlementInfo {
    pub user_count: Option<i32>,
    pub response_count: Option<i32>,
    pub question_per_form_count: Option<i32>,
    pub form_count: Option<i32>,
    pub team_count: Option<i32>,
    pub branding_count: Option<i32>,
    pub export: bool,
    pub import_keys: bool,
    pub subdomain: bool,
    pub crypto_details: bool,
    pub audit: bool,
    pub priority_support: bool,
    pub oidc: bool,
    pub submission_auto_delete: bool,
    pub form_captcha: bool,
}

impl Default for APIEntitlementInfo {
    fn default() -> Self {
        Self {
            user_count: Some(1),
            response_count: Some(250),
            question_per_form_count: Some(20),
            form_count: Some(10),
            team_count: Some(0),
            branding_count: Some(0),
            export: false,
            import_keys: false,
            subdomain: false,
            crypto_details: false,
            audit: false,
            priority_support: false,
            oidc: false,
            submission_auto_delete: false,
            form_captcha: false,
        }
    }
}

impl APIEntitlementInfo {
    pub fn to_active_model(&self) -> organisation_feature_entitlement::ActiveModel {
        organisation_feature_entitlement::ActiveModel {
            user_count: Set(self.user_count),
            response_count: Set(self.response_count),
            question_per_form_count: Set(self.question_per_form_count),
            form_count: Set(self.form_count),
            team_count: Set(self.team_count),
            branding_count: Set(self.branding_count),
            export: Set(self.export),
            import_keys: Set(self.import_keys),
            subdomain: Set(self.subdomain),
            crypto_details: Set(self.crypto_details),
            audit: Set(self.audit),
            priority_support: Set(self.priority_support),
            oidc: Set(self.oidc),
            submission_auto_delete: Set(self.submission_auto_delete),
            form_captcha: Set(self.form_captcha),
            ..Default::default()
        }
    }
}
