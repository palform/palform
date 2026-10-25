use actix_web::web::{Data, Json, Path};
use apistos::{api_operation, ApiComponent};
use palform_client_common::{
    errors::error::{APIError, APIInternalErrorResult},
    form_management::{question_group::APIQuestionGroup, question_types::APIQuestion},
};
use palform_tsid::{
    resources::{IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use schemars::JsonSchema;
use sea_orm::{AccessMode, DatabaseConnection, IsolationLevel, TransactionTrait};
use serde::Deserialize;

use crate::{
    auth::rbac::requests::APITokenTeamEditorFromForm, entity_managers::questions::QuestionManager,
};

#[derive(JsonSchema, Deserialize, ApiComponent)]
pub struct APISaveQuestionsRequest {
    /// The full list of questions. Their group ID must be one of the specified groups.
    questions: Vec<APIQuestion>,
    /// The full list of question groups.
    groups: Vec<APIQuestionGroup>,
}

#[derive(Deserialize, JsonSchema, ApiComponent)]
pub struct QuestionsSavePath {
    org_id: PalformDatabaseID<IDOrganisation>,
    form_id: PalformDatabaseID<IDForm>,
}

/// Set the questions and grouos of a form.
///
/// This configures the questions, question groups, and stepping configuration for a form all in one
/// go. It diffs the questions and question groups you provide with the current state of the form in
/// the DB. It creates questions/question groups where they don't yet exist (by ID), updates their
/// metadata if they have changed, updates their positions if they have changed, and deletes them if
/// they are not in the request.
///
/// Therefore, the questions and groups must be provided in full on each request. A question that
/// exists in the form but is not sent in this request will be deleted. The comparison checking
/// works by ID.
///
/// Note that deleted questions will be completely deleted immediately and will not be served to new
/// respondents. Existing responses will retain the values for deleted questions of course, since
/// Palform can't view or edit end-to-end-encrypted response data.
#[api_operation(tag = "Questions", operation_id = "questions.save")]
pub async fn questions_save(
    path: Path<QuestionsSavePath>,
    data: Json<APISaveQuestionsRequest>,
    _token: APITokenTeamEditorFromForm,
    db: Data<DatabaseConnection>,
) -> Result<(), APIError> {
    let txn = db
        .begin_with_config(
            Some(IsolationLevel::RepeatableRead),
            Some(AccessMode::ReadWrite),
        )
        .await
        .map_internal_error()?;

    #[cfg(feature = "saas")]
    {
        use crate::billing::entitlement::INTERNALBillingEntitlementManager;

        let billing = INTERNALBillingEntitlementManager::new(path.org_id);
        let org_entitlement = billing
            .get_org_entitlement(&txn)
            .await
            .map_internal_error()?;
        if org_entitlement
            .question_per_form_count
            .is_some_and(|v| data.questions.len() as i32 > v)
        {
            return Err(
                APIError::SubscriptionLimit("Cannot exceed question limit".to_string()).into(),
            );
        }
    }

    QuestionManager::save_questions_and_groups(
        &txn,
        path.form_id,
        data.groups.clone(),
        data.questions.clone(),
    )
    .await
    .map_err(|e| APIError::report_internal_error("save all questions and groups in form", e))?;

    txn.commit().await.map_internal_error()?;
    Ok(())
}
