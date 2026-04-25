use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::organisation_invite::APIOrganisationInvite,
    auth::rbac::requests::APITokenOrgAdmin,
    entity_managers::organisation_invites::OrganisationInviteManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationInvitesListPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Organisation Invites", operation_id = "organisation.invites.list")]
pub async fn organisation_invites_list(
    path: Path<OrganisationInvitesListPath>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIOrganisationInvite>>, APIError> {
    OrganisationInviteManager::list(db.as_ref(), path.org_id)
        .await
        .map(Json)
        .map_err(|e| e.to_internal_error())
}
