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
    pub display_name: String,
    pub created_at: DateTime<Utc>,
    pub subdomain: Option<String>,
    pub uses_oidc: bool,
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
