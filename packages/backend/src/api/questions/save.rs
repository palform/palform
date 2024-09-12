use palform_client_common::{
    errors::error::{APIError, APIErrorWithStatus, APIInternalErrorResult},
    form_management::{question_group::APIQuestionGroup, question_types::APIQuestion},
};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use rocket::{post, serde::json::Json, State};
use rocket_okapi::{
    okapi::schemars::{self, JsonSchema},
    openapi,
};
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    auth::rbac::requests::APITokenTeamEditorFromForm,
    billing::entitlement::INTERNALBillingEntitlementManager,
    entity_managers::questions::QuestionManager,
};

#[derive(JsonSchema, Deserialize)]
pub struct APISaveQuestionsRequest {
    questions: Vec<APIQuestion>,
    groups: Vec<APIQuestionGroup>,
}

#[openapi(tag = "Questions", operation_id = "questions.save")]
#[post("/users/me/orgs/<org_id>/forms/<form_id>/save", data = "<data>")]
pub async fn handler(
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
    data: Json<APISaveQuestionsRequest>,
    _token: APITokenTeamEditorFromForm,
    db: &State<DatabaseConnection>,
) -> Result<(), APIErrorWithStatus> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    let billing = INTERNALBillingEntitlementManager::new(org_id);
    let org_entitlement = billing
        .get_org_entitlement(&txn)
        .await
        .map_internal_error()?;
    if org_entitlement
        .question_per_form_count
        .is_some_and(|v| data.questions.len() as i32 > v)
    {
        return Err(APIError::SubscriptionLimit(format!("Cannot exceed question limit")).into());
    }

    QuestionManager::save_questions_and_groups(
        &txn,
        form_id,
        data.groups.clone(),
        data.questions.clone(),
    )
    .await
    .map_err(|e| APIError::report_internal_error("save all questions and groups in form", e))?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
