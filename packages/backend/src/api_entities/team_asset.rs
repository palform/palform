use chrono::NaiveDateTime;
use palform_entities::team_asset;
use palform_tsid::resources::IDTeamAsset;
use palform_tsid::tsid::PalformDatabaseID;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Serialize;

#[derive(JsonSchema, Serialize)]
pub struct APITeamAsset {
    id: PalformDatabaseID<IDTeamAsset>,
    created_at: NaiveDateTime,
    pub url: String,
}

impl APITeamAsset {
    pub fn from(rec: team_asset::Model, url: String) -> Self {
        Self {
            id: rec.id,
            created_at: rec.created_at,
            url,
        }
    }
}
