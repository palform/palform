use palform_tsid::{resources::{IDOrganisation, IDOrganisationInvite}, tsid::PalformDatabaseID};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::DatabaseConnection;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::organisation_invite::APIOrganisationInvitePreview,
    auth::tokens::{APIAuthToken, APIAuthTokenSourcePersonal},
    entity_managers::organisation_invites::OrganisationInviteManager,
};

#[openapi(
    tag = "Organisation Invites",
    operation_id = "organisation.invites.preview"
)]
#[get("/users/me/orgs/<org_id>/invites/<invite_id>/preview")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    invite_id: PalformDatabaseID<IDOrganisationInvite>,
    _token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: &State<DatabaseConnection>,
) -> Result<Json<APIOrganisationInvitePreview>, (Status, Json<APIError>)> {
    let preview = OrganisationInviteManager::preview(db.inner(), invite_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    if preview.org_id != org_id {
        return Err(APIError::NotFound.into());
    }

    Ok(Json(preview))
}
