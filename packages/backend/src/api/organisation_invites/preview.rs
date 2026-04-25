use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::{
    resources::{IDOrganisation, IDOrganisationInvite},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::organisation_invite::APIOrganisationInvitePreview,
    auth::tokens::{APIAuthToken, APIAuthTokenSourcePersonal},
    entity_managers::organisation_invites::OrganisationInviteManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationInvitesPreviewPath {
    org_id: PalformDatabaseID<IDOrganisation>,
    invite_id: PalformDatabaseID<IDOrganisationInvite>,
}

#[api_operation(tag = "Organisation Invites", operation_id = "organisation.invites.preview")]
pub async fn organisation_invites_preview(
    path: Path<OrganisationInvitesPreviewPath>,
    _token: APIAuthToken<APIAuthTokenSourcePersonal>,
    db: Data<DatabaseConnection>,
) -> Result<Json<APIOrganisationInvitePreview>, APIError> {
    let preview = OrganisationInviteManager::preview(db.as_ref(), path.invite_id)
        .await
        .map_err(|e| e.to_internal_error())?
        .ok_or(APIError::NotFound)?;

    if preview.org_id != path.org_id {
        return Err(APIError::NotFound.into());
    }

    Ok(Json(preview))
}
