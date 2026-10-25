use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_entities::fill_access_token;
use palform_tsid::resources::{IDFillAccessToken, IDForm, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

/// A token with which end-users can access form filling related API endpoints for a specific form
/// only.
#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIFillToken {
    pub id: PalformDatabaseID<IDFillAccessToken>,
    form_id: PalformDatabaseID<IDForm>,
    created_at: DateTime<Utc>,
    /// Optionally, an expiry time after which the token will no longer work
    expires_at: Option<DateTime<Utc>>,
    /// An internal nickname to allow segmenting submissions by fill access token
    nickname: String,
    /// Optionally, a shortened path through which the form can be filled. This requires the
    /// organisation to have a subdomain configured. The form will be available at
    /// `https://<subdomain>.palform.app/<short_link>`. Fill access tokens with short links can
    /// still be accessed at the long path, too.
    short_link: Option<String>,
}

impl From<fill_access_token::Model> for APIFillToken {
    fn from(value: fill_access_token::Model) -> Self {
        Self {
            id: value.id,
            form_id: value.form_id,
            created_at: value.created_at.to_utc(),
            expires_at: value.expires_at.map(|v| v.to_utc()),
            nickname: value.nickname,
            short_link: value.short_link,
        }
    }
}

#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIExchangedShortLink {
    fill_token_id: PalformDatabaseID<IDFillAccessToken>,
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    pub expires_at: Option<DateTime<Utc>>,
}
