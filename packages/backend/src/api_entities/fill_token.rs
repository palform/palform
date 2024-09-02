use chrono::NaiveDateTime;
use palform_entities::fill_access_token;
use palform_tsid::resources::{IDFillAccessToken, IDForm, IDOrganisation};
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::Serialize;

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIFillToken {
    id: PalformDatabaseID<IDFillAccessToken>,
    form_id: PalformDatabaseID<IDForm>,
    created_at: NaiveDateTime,
    expires_at: Option<NaiveDateTime>,
    nickname: String,
    short_link: Option<String>,
}

impl From<fill_access_token::Model> for APIFillToken {
    fn from(value: fill_access_token::Model) -> Self {
        Self {
            id: value.id,
            form_id: value.form_id,
            created_at: value.created_at,
            expires_at: value.expires_at,
            nickname: value.nickname,
            short_link: value.short_link,
        }
    }
}

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIExchangedShortLink {
    fill_token_id: PalformDatabaseID<IDFillAccessToken>,
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    pub expires_at: Option<NaiveDateTime>,
}
