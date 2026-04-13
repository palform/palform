use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    auth::rbac::requests::APITokenOrgViewer, auth::tokens::APIAuthTokenSource,
    entity_managers::organisation_members::OrganisationMembersManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationMembersAmIAdminPath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(
    tag = "Organisation Members",
    operation_id = "organisation.members.am_i_admin"
)]
pub async fn organisation_members_am_i_admin(
    path: Path<OrganisationMembersAmIAdminPath>,
    token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<bool>, APIError> {
    let resp =
        OrganisationMembersManager::get_is_admin(db.as_ref(), path.org_id, token.get_user_id())
            .await
            .map_internal_error()?
            .ok_or(APIError::NotFound)?;

    Ok(Json(resp))
}
