use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use chrono::{DateTime, Utc};
use palform_client_common::errors::error::{APIError, APIInternalErrorResult};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use schemars::JsonSchema;
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::submission::APISubmissionCountPerForm, auth::rbac::requests::APITokenOrgViewer,
    auth::tokens::APIAuthTokenSource, entity_managers::submission::SubmissionManager,
};

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SubmissionCountSinceRequest {
    since: DateTime<Utc>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct SubmissionsNumSincePath {
    org_id: PalformDatabaseID<IDOrganisation>,
}

#[api_operation(tag = "Submissions", operation_id = "submissions.num_since")]
pub async fn submissions_num_since(
    path: Path<SubmissionsNumSincePath>,
    data: Json<SubmissionCountSinceRequest>,
    token: APITokenOrgViewer,
    db: Data<DatabaseConnection>,
) -> Result<Json<Vec<APISubmissionCountPerForm>>, APIError> {
    let count = SubmissionManager::submission_count_in_my_forms_since(
        db.as_ref(),
        path.org_id,
        token.get_user_id(),
        data.since,
    )
    .await
    .map_internal_error()?;

    Ok(Json(count))
}
