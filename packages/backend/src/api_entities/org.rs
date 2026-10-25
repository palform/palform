use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

use crate::api_entities::billing::plan::APIBillingSubscription;

#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIOrganisation {
    pub id: PalformDatabaseID<IDOrganisation>,
    /// The name of the organisation, visible to admins and anyone filling any of the organisation's
    /// forms.
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    /// Optionally, a subdomain used by the organisation for short form links and possibly for
    /// authentication via OIDC (if configured).
    pub subdomain: Option<String>,
    /// Whether the organisation uses OIDC authentication.
    pub uses_oidc: bool,
    /// Whether to allow the organisation to go over its response limit and therefore be charged for
    /// additional responses.
    #[deprecated(
        note = "Palform now uses an unlimited-response model where overage no longer needs to be tracked."
    )]
    pub billing_allow_overage: bool,
}

#[derive(Serialize, JsonSchema, ApiComponent)]
pub struct APIOrganisationManifest {
    pub form_count: u64,
    pub question_count: u64,
    pub submission_count: u64,
    pub member_count: u64,
    pub team_count: u64,
    pub team_asset_count: u64,
    pub branding_count: u64,
    pub audit_log_count: u64,
    pub active_subscriptions: Vec<APIBillingSubscription>,
}
