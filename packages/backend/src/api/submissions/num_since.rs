use chrono::{DateTime, Utc};
use palform_client_common::errors::error::{APIErrorWithStatus, APIInternalErrorResult};
use palform_tsid::resources::IDOrganisation;
use palform_tsid::tsid::PalformDatabaseID;
use rocket::{post, serde::json::Json, State};
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{okapi::schemars, openapi};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::submission::APISubmissionCountPerForm, auth::rbac::requests::APITokenOrgViewer,
    auth::tokens::APIAuthTokenSource, entity_managers::submission::SubmissionManager,
};

#[derive(Deserialize, JsonSchema)]
pub struct SubmissionCountSinceRequest {
    since: DateTime<Utc>,
}

#[openapi(tag = "Submissions", operation_id = "submissions.num_since")]
#[post("/users/me/orgs/<org_id>/submissions", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    data: Json<SubmissionCountSinceRequest>,
    token: APITokenOrgViewer,
    db: &State<DatabaseConnection>,
) -> Result<Json<Vec<APISubmissionCountPerForm>>, APIErrorWithStatus> {
    let count = SubmissionManager::submission_count_in_my_forms_since(
        db.inner(),
        org_id,
        token.get_user_id(),
        data.since,
    )
    .await
    .map_internal_error()?;

    Ok(Json(count))
}
