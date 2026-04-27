use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use chrono::Duration;
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    actix_util::from_org_id::FromOrgId,
    api_entities::organisation_deletion_request::APIOrganisationDeletionRequest,
    auth::rbac::requests::APITokenOrgAdmin, config::Config,
    entity_managers::organisation_deletion_request::OrganisationDeletionRequestManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct OrganisationsDeletionRequestsListPath {
    #[allow(dead_code)]
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(
    tag = "Organisation Deletion Requests",
    operation_id = "organisation_deletion_requests.list"
)]
pub async fn organisation_deletion_requests_list(
    _path: Path<OrganisationsDeletionRequestsListPath>,
    _token: APITokenOrgAdmin,
    db: Data<DatabaseConnection>,
    org_deletion_request_manager: FromOrgId<OrganisationDeletionRequestManager>,
    config: Data<Config>,
) -> Result<Json<Vec<APIOrganisationDeletionRequest>>, APIError> {
    let requests = org_deletion_request_manager
        .list(db.as_ref())
        .await
        .map_internal_error()?;

    let requests = requests
        .iter()
        .map(|r| APIOrganisationDeletionRequest {
            id: r.id,
            organisation_id: r.organisation_id,
            user_id: r.user_id,
            include_user: r.include_user,
            status: r.status.clone(),
            reason: r.reason.parse().expect("cancel reason"),
            created_at: r.created_at.to_utc(),
            deletion_at: (r.created_at
                + Duration::hours(i64::from(config.organisation_deletion_grace_period_hours)))
            .to_utc(),
        })
        .collect();

    Ok(Json(requests))
}
