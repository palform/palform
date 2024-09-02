use chrono::{Duration, Utc};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{http::Status, post, serde::json::Json, State};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::organisation_invite::APIOrganisationInvite,
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::organisation_invites::OrganisationInviteManager,
};

#[derive(Deserialize, JsonSchema)]
pub struct NewOrganisationInviteRequest {
    pub expires_in_seconds: u32,
    pub single_use: bool,
}

#[openapi(
    tag = "Organisation Invites",
    operation_id = "organisation.invites.create"
)]
#[post("/users/me/orgs/<org_id>/invites", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Json<NewOrganisationInviteRequest>,
    _token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIOrganisationInvite>, (Status, Json<APIError>)> {
    OrganisationInviteManager::create(
        db.inner(),
        org_id,
        (Utc::now() + Duration::seconds(i64::from(data.expires_in_seconds))).naive_utc(),
        data.single_use,
    )
    .await
    .map(|e| Json(e.into()))
    .map_err(|e| e.to_internal_error())
}
