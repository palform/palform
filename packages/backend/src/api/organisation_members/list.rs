use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{get, http::Status, serde::json::Json, State};
use rocket_okapi::openapi;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};

use crate::{
    api::error::{APIError, APIInternalError},
    api_entities::organisation_member::APIOrgMember,
    auth::rbac::requests::APITokenOrgViewer,
    entity_managers::organisation_members::OrganisationMembersManager,
};

#[openapi(
    tag = "Organisation Members",
    operation_id = "organisation.members.list"
)]
#[get("/users/me/orgs/<org_id>/members")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    _token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APIOrgMember>>, (Status, Json<APIError>)> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadOnly),
        )
        .await
        .map_err(|e| e.to_internal_error())?;

    let members = OrganisationMembersManager::list_all(&txn, org_id)
        .await
        .map_err(|e| e.to_internal_error())?;

    Ok(Json(members))
}
