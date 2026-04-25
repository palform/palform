use actix_web::web::{Data, Path};
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
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::organisation_invites::OrganisationInviteManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationInvitesDeletePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    invite_id: PalformDatabaseID<IDOrganisationInvite>,
}

#[api_operation(tag = "Organisation Invites", operation_id = "organisation.invites.delete")]
pub async fn organisation_invites_delete(
    path: Path<OrganisationInvitesDeletePath>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    if !OrganisationInviteManager::verify_invite_org(db.as_ref(), path.invite_id, path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?
    {
        return Err(APIError::NotFound.into());
    }

    OrganisationInviteManager::delete(db.as_ref(), path.invite_id)
        .await
        .map_err(|e| e.to_internal_error())?;
    Ok(())
}
