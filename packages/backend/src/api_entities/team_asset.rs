use apistos::ApiComponent;
use chrono::{DateTime, Utc};
use palform_entities::team_asset;
use palform_tsid::resources::IDTeamAsset;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use serde::Serialize;

#[derive(JsonSchema, Serialize, ApiComponent)]
pub struct APITeamAsset {
    id: PalformDatabaseID<IDTeamAsset>,
    created_at: DateTime<Utc>,
    pub url: String,
}

impl APITeamAsset {
    pub fn from(rec: team_asset::Model, url: String) -> Self {
        Self {
            id: rec.id,
            created_at: rec.created_at.to_utc(),
            url,
        }
    }
}
