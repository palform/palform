use std::collections::HashMap;

use chrono::{Duration, Utc};
use palform_client_common::form_management::{
    form_end::APIFormEndConfiguration,
    question_group::{
        APIQuestionGroupStepStrategy, APIQuestionGroupStepStrategyJumpCase,
        APIQuestionGroupStepStrategyJumpCaseCondition,
    },
};
use palform_entities::{
    deleted_submission, form, organisation, prelude::*, question, question_group, submission, team,
    team_membership,
};
use palform_migration::all;
use palform_tsid::{
    resources::{
        IDAdminUser, IDForm, IDFormBranding, IDOrganisation, IDQuestion, IDQuestionGroup,
        IDSubmission, IDTeam,
    },
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, FromQueryResult, JoinType,
    PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, Set,
};
use thiserror::Error;

use crate::api_entities::{
    form::{APIForm, APIFormWithQuestions, APIFrontendForm},
    form_brandings::APIFormBranding,
};

use super::{
    billing_entitlement_proxy::BillingEntitlementCountTrait, form_brandings::FormBrandingManager,
    question_groups::QuestionGroupManager, questions::QuestionManager,
};

#[derive(Error, Debug)]
pub enum GetFormError {
    #[error("Database: {0}")]
    DBError(#[from] DbErr),
    #[error("Decode: {0}")]
    DecodeError(#[from] serde_json::Error),
    #[error("Form not found")]
    NotFound,
}

#[derive(FromQueryResult)]
struct FormOnlyTeamId {
    team_id: PalformDatabaseID<IDTeam>,
}

impl BillingEntitlementCountTrait for FormManager {
    async fn billing_count<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<u64, DbErr> {
        Form::find()
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .filter(team::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await
    }
}

pub struct FormManager;

impl FormManager {
    pub async fn list_forms_in_my_teams<T: ConnectionTrait>(
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Vec<APIForm>, DbErr> {
        Form::find()
            .join(JoinType::LeftJoin, form::Relation::Submission.def())
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .join(JoinType::InnerJoin, team::Relation::Organisation.def())
            .join(JoinType::InnerJoin, team::Relation::TeamMembership.def())
            .filter(team::Column::OrganisationId.eq(org_id))
            .filter(team_membership::Column::UserId.eq(user_id))
            .column_as(submission::Column::Id.count(), "response_count")
            .group_by(form::Column::Id)
            .into_model()
            .all(conn)
            .await
    }

    pub async fn get_with_questions<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<APIFormWithQuestions, GetFormError> {
        let form = Form::find_by_id(form_id)
            .into_model::<APIFrontendForm>()
            .one(conn)
            .await?
            .ok_or(GetFormError::NotFound)?;

        let questions = QuestionManager::get_all_for_form(conn, form_id)
            .await
            .map_err(|e| match e {
                super::questions::GetQuestionError::Decode(e) => GetFormError::DecodeError(e),
                super::questions::GetQuestionError::DB(e) => GetFormError::DBError(e),
            })?;

        let groups = QuestionGroupManager::list_all_for_form(conn, form_id)
            .await
            .map_err(GetFormError::DBError)?;

        let mut branding: Option<APIFormBranding> = None;
        if let Some(branding_id) = &form.branding_id {
            branding = Some(
                FormBrandingManager::get_by_id(conn, *branding_id)
                    .await
                    .map_err(GetFormError::DBError)?
                    .ok_or(GetFormError::DBError(DbErr::RecordNotFound(
                        "branding ID not found".to_string(),
                    )))?,
            );
        }

        let org_name: String = Form::find_by_id(form_id)
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .join(JoinType::InnerJoin, team::Relation::Organisation.def())
            .select_only()
            .column(organisation::Column::DisplayName)
            .into_tuple()
            .one(conn)
            .await?
            .ok_or(GetFormError::NotFound)?;

        Ok(APIFormWithQuestions {
            form,
            questions,
            groups,
            branding,
            org_name,
        })
    }

    pub async fn get_by_id<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<Option<APIForm>, DbErr> {
        Form::find_by_id(form_id)
            .join_rev(JoinType::LeftJoin, submission::Relation::Form.def())
            .column_as(submission::Column::Id.count(), "response_count")
            .group_by(form::Column::Id)
            .into_model::<APIForm>()
            .one(conn)
            .await
    }

    pub async fn get_form_team_id<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<PalformDatabaseID<IDTeam>, DbErr> {
        let resp = Form::find_by_id(form_id)
            .into_model::<FormOnlyTeamId>()
            .one(conn)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Required form ID not found".to_string(),
            ))?;

        Ok(resp.team_id)
    }

    pub async fn get_plain_by_id<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<Option<form::Model>, DbErr> {
        Form::find_by_id(form_id).one(conn).await
    }

    pub async fn get_captcha_required<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<bool, DbErr> {
        Form::find_by_id(form_id)
            .select_only()
            .column(form::Column::EnableCaptcha)
            .into_tuple()
            .one(conn)
            .await
            .map(|v| v.unwrap_or(false))
    }

    pub async fn verify_form_org<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<bool, DbErr> {
        let actual_org_id: Option<PalformDatabaseID<IDOrganisation>> = Form::find_by_id(form_id)
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .select_only()
            .column(team::Column::OrganisationId)
            .into_tuple()
            .one(conn)
            .await?;

        Ok(actual_org_id == Some(org_id))
    }

    pub fn serialize_end_configuration(
        config: APIFormEndConfiguration,
    ) -> Result<serde_json::Value, DbErr> {
        serde_json::to_value(config).map_err(|e| DbErr::Json(e.to_string()))
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
        editor_name: String,
        title: String,
        branding_id: Option<PalformDatabaseID<IDFormBranding>>,
        one_question_per_page: bool,
    ) -> Result<APIForm, DbErr> {
        let initial_end_config =
            Self::serialize_end_configuration(APIFormEndConfiguration::default())?;
        let new_form = form::ActiveModel {
            id: Set(PalformDatabaseID::<IDForm>::random()),
            editor_name: Set(editor_name),
            title: Set(Some(title)),
            team_id: Set(team_id),
            branding_id: Set(branding_id),
            end_configuration: Set(initial_end_config),
            one_question_per_page: Set(one_question_per_page),
            ..Default::default()
        };
        let form = new_form.insert(conn).await?;
        let form = Self::get_by_id(conn, form.id)
            .await?
            .ok_or(DbErr::Custom("Form just created did not exist".to_string()))?;
        Ok(form)
    }

    pub async fn delete<T: ConnectionTrait>(
        conn: &T,
        id: PalformDatabaseID<IDForm>,
    ) -> Result<(), DbErr> {
        Form::delete_by_id(id).exec(conn).await.map(|_| ())
    }

    pub async fn change_form_team<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        new_team_id: PalformDatabaseID<IDTeam>,
    ) -> Result<(), DbErr> {
        let updated_form = form::ActiveModel {
            id: Set(form_id),
            team_id: Set(new_team_id),
            // The new team won't have the current branding so we need to reset it
            branding_id: Set(None),
            ..Default::default()
        };
        updated_form.update(conn).await.map(|_| ())
    }

    pub async fn clone<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        into_team_id: PalformDatabaseID<IDTeam>,
        add_clone_to_name: bool,
    ) -> Result<APIForm, DbErr> {
        let original_form = Form::find_by_id(form_id)
            .one(conn)
            .await?
            .ok_or(DbErr::RecordNotFound("Form not found".to_string()))?;

        let original_qgs = QuestionGroup::find()
            .filter(question_group::Column::FormId.eq(original_form.id))
            .all(conn)
            .await?;

        let original_questions = Question::find()
            .join(JoinType::InnerJoin, question::Relation::QuestionGroup.def())
            .filter(question_group::Column::FormId.eq(original_form.id))
            .all(conn)
            .await?;

        let new_form_id = PalformDatabaseID::<IDForm>::random();
        let new_form = form::ActiveModel {
            id: Set(new_form_id),
            editor_name: Set(if add_clone_to_name {
                format!("{} (clone)", original_form.editor_name)
            } else {
                original_form.editor_name
            }),
            title: Set(original_form.title),
            team_id: Set(into_team_id),
            end_configuration: Set(original_form.end_configuration),
            one_question_per_page: Set(original_form.one_question_per_page),
            ..Default::default()
        };
        new_form.insert(conn).await?;

        let mut old_to_new_question_groups =
            HashMap::<PalformDatabaseID<IDQuestionGroup>, PalformDatabaseID<IDQuestionGroup>>::new(
            );
        for question_group in &original_qgs {
            let new_question_group_id = PalformDatabaseID::<IDQuestionGroup>::random();
            let new_question_group = question_group::ActiveModel {
                id: Set(new_question_group_id),
                form_id: Set(new_form_id),
                title: Set(question_group.title.clone()),
                description: Set(question_group.description.clone()),
                step_strategy: Set(question_group.step_strategy.clone()),
                position: Set(question_group.position),
            };

            new_question_group.insert(conn).await?;
            old_to_new_question_groups.insert(question_group.id, new_question_group_id);
        }

        let mut old_to_new_questions =
            HashMap::<PalformDatabaseID<IDQuestion>, PalformDatabaseID<IDQuestion>>::new();
        for question in original_questions {
            let new_group_id = old_to_new_question_groups
                .get(&question.group_id)
                .ok_or(DbErr::RecordNotFound("This cannot happen".to_string()))?;

            let new_question_id = PalformDatabaseID::<IDQuestion>::random();
            let new_question = question::ActiveModel {
                id: Set(new_question_id),
                group_id: Set(*new_group_id),
                title: Set(question.title),
                description: Set(question.description),
                configuration: Set(question.configuration),
                position: Set(question.position),
                required: Set(question.required),
                internal_name: Set(question.internal_name),
            };

            new_question.insert(conn).await?;
            old_to_new_questions.insert(question.id, new_question_id);
        }

        for question_group in &original_qgs {
            let new_question_group_id =
                old_to_new_question_groups
                    .get(&question_group.id)
                    .ok_or(DbErr::RecordNotFound(
                        "Newly created question group ID not found".to_string(),
                    ))?;

            let step_strategy: APIQuestionGroupStepStrategy =
                serde_json::from_value(question_group.step_strategy.clone())
                    .map_err(|e| DbErr::Custom(format!("Decode step strategy: {}", e)))?;

            if let APIQuestionGroupStepStrategy::JumpToSection(cases) = step_strategy {
                let mut new_cases = Vec::<APIQuestionGroupStepStrategyJumpCase>::new();
                for case in cases {
                    let mut new_target_group_id = case.target_group_id;
                    if let Some(current_target_group_id) = case.target_group_id {
                        new_target_group_id = Some(
                            old_to_new_question_groups
                                .get(&current_target_group_id)
                                .ok_or(DbErr::Custom(
                                    "Question Group in step strategy not found".to_string(),
                                ))?
                                .to_owned(),
                        );
                    }

                    let mut new_condition_items =
                        Vec::<APIQuestionGroupStepStrategyJumpCaseCondition>::new();
                    for condition in case.conditions.get_items() {
                        let new_question_id = old_to_new_questions
                            .get(&condition.question_id)
                            .ok_or(DbErr::Custom(
                                "Question in step strategy condition not found".to_string(),
                            ))?
                            .to_owned();

                        new_condition_items.push(APIQuestionGroupStepStrategyJumpCaseCondition {
                            question_id: new_question_id,
                            matcher: condition.matcher.clone(),
                        })
                    }

                    new_cases.push(APIQuestionGroupStepStrategyJumpCase {
                        target_group_id: new_target_group_id,
                        conditions: case.conditions.clone_with(new_condition_items),
                    })
                }

                let new_step_strategy =
                    serde_json::to_value(APIQuestionGroupStepStrategy::JumpToSection(new_cases))
                        .map_err(|e| DbErr::Custom(format!("Re-encode step strategy: {}", e)))?;

                let updated_question_group = question_group::ActiveModel {
                    id: Set(new_question_group_id.to_owned()),
                    step_strategy: Set(new_step_strategy),
                    ..Default::default()
                };
                updated_question_group.update(conn).await?;
            }
        }

        let newly_created_form =
            FormManager::get_by_id(conn, new_form_id)
                .await?
                .ok_or(DbErr::RecordNotFound(
                    "Newly created form did not exist :/".to_string(),
                ))?;

        Ok(newly_created_form)
    }

    pub async fn set_auto_delete<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        auto_delete_after_days: Option<i32>,
    ) -> Result<(), DbErr> {
        let updated_form = form::ActiveModel {
            id: Set(form_id),
            auto_delete_submission_after_days: Set(auto_delete_after_days),
            ..Default::default()
        };
        updated_form.update(conn).await.map(|_| ())
    }

    pub async fn delete_all_old_submissions<T: ConnectionTrait>(conn: &T) -> Result<(), DbErr> {
        let forms: Vec<(PalformDatabaseID<IDForm>, Option<i32>)> = Form::find()
            .join(JoinType::InnerJoin, form::Relation::Submission.def())
            .filter(form::Column::AutoDeleteSubmissionAfterDays.is_not_null())
            .select_only()
            .column(form::Column::Id)
            .column(form::Column::AutoDeleteSubmissionAfterDays)
            .into_tuple()
            .all(conn)
            .await?;

        for (form_id, days) in forms {
            let days = days.ok_or(DbErr::RecordNotFound(
                "Days was null despite filter".to_string(),
            ))?;
            let cut_off_date = (Utc::now() - Duration::days(days.into())).naive_utc();

            let delete_ids: Vec<PalformDatabaseID<IDSubmission>> = Submission::find()
                .filter(all![
                    submission::Column::FormId.eq(form_id),
                    submission::Column::CreatedAt.lt(cut_off_date)
                ])
                .select_only()
                .column(submission::Column::Id)
                .into_tuple()
                .all(conn)
                .await?;

            for delete_id in &delete_ids {
                let new_deletion = deleted_submission::ActiveModel {
                    id: Set(*delete_id),
                    form_id: Set(form_id),
                    ..Default::default()
                };
                new_deletion.insert(conn).await?;
            }

            if !delete_ids.is_empty() {
                Submission::delete_many()
                    .filter(submission::Column::Id.is_in(delete_ids))
                    .exec(conn)
                    .await?;
            }
        }

        Ok(())
    }
}
