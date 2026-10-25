use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::organisation_member::APIOrgMemberServiceAccount,
    auth::rbac::requests::APITokenOrgViewer,
    entity_managers::organisation_members::OrganisationMembersManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationMembersListServiceAccountPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(
    tag = "Organisation Members",
    operation_id = "organisation.members.list_service_account"
)]
pub async fn organisation_members_list_service_account(
    path: Path<OrganisationMembersListServiceAccountPath>,
    _token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APIOrgMemberServiceAccount>>, APIError> {
    let members = OrganisationMembersManager::list_service_account(db.as_ref(), path.org_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    Ok(Json(members))
}
