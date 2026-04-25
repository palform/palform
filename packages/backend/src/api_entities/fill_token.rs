use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_entities::fill_access_token;
use palform_tsid::resources::{IDFillAccessToken, IDForm, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, FromQueryResult, ApiComponent)]
pub struct APIFillToken {
    pub id: PalformDatabaseID<IDFillAccessToken>,
    form_id: PalformDatabaseID<IDForm>,
    created_at: DateTime<Utc>,
    expires_at: Option<DateTime<Utc>>,
    nickname: String,
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
