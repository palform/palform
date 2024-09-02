use palform_client_common::{
    errors::error::{APIError, APIErrorWithStatus},
    form_management::question_types::APIQuestion,
};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDQuestionGroup},
    tsid::PalformDatabaseID,
};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::{okapi::schemars::{self, JsonSchema}, openapi};
use sea_orm::DatabaseConnection;
use serde::Deserialize;

use crate::{
    api_entities::billing::entitlement::APIEntitlementRequest,
    auth::rbac::requests::APITokenTeamEditorFromForm,
    entity_managers::{
        billing_entitlement_proxy::BillingEntitlementManager,
        questions::{CreateQuestionError, QuestionManager},
    },
    rocket_util::from_org_id::FromOrgId,
};

#[derive(Deserialize, JsonSchema)]
pub struct CreateQuestionRequest {
    question_type: String,
    position: i32,
}

#[openapi(tag = "Questions", operation_id = "questions.create")]
#[post(
    "/users/me/orgs/<_org_id>/forms/<form_id>/groups/<question_group_id>/questions",
    data = "<data>"
)]
pub async fn handler(
    _org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    question_group_id: PalformDatabaseID<IDQuestionGroup>,
    data: Json<CreateQuestionRequest>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
    billing: FromOrgId<BillingEntitlementManager>,
) -> Result<Json<APIQuestion>, APIErrorWithStatus> {
    billing
        .check_entitlement(
            db.inner(),
            APIEntitlementRequest::QuestionPerFormCount(form_id),
        )
        .await?;

    let new_question = QuestionManager::create_default_question(
        db.inner(),
        question_group_id,
        data.question_type.clone(),
        data.position,
    )
    .await
    .map_err(|e| match e {
        CreateQuestionError::CreateDefault(_) => APIError::BadRequest(e.to_string()).into(),
        _ => APIError::report_internal_error("create default question", e),
    })?;

    Ok(Json(new_question))
}
