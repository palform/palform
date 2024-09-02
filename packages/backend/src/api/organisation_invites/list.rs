use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::organisation_invite::APIOrganisationInvite,
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::organisation_invites::OrganisationInviteManager,
};

#[openapi(
    tag = "Organisation Invites",
    operation_id = "organisation.invites.list"
)]
#[get("/users/me/orgs/<org_id>/invites")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgAdmin,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIOrganisationInvite>>, (Status, Json<APIError>)> {
    OrganisationInviteManager::list(db.inner(), org_id)
        .await
        .map(Json)
        .map_err(|e| e.to_internal_error())
}
