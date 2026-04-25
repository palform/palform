use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_entities::organisation_invite;
use palform_tsid::resources::{IDOrganisation, IDOrganisationInvite};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIOrganisationInvite {
    pub id: PalformDatabaseID<IDOrganisationInvite>,
    pub single_use: bool,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

impl From<organisation_invite::Model> for APIOrganisationInvite {
    fn from(value: organisation_invite::Model) -> Self {
        Self {
            id: value.id,
            single_use: value.single_use,
            created_at: value.created_at.to_utc(),
            expires_at: value.expires_at.to_utc(),
        }
    }
}

#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIOrganisationInvitePreview {
    pub org_id: PalformDatabaseID<IDOrganisation>,
    pub org_display_name: String,
    pub expires_at: DateTime<Utc>,
}
