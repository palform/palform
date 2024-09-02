use chrono::{Duration, Utc};
use palform_client_common::form_management::form_end::APIFormEndConfiguration;
use palform_entities::{form, organisation, prelude::*, submission, team, team_membership};
use palform_tsid::{
    resources::{IDAdminUser, IDForm, IDFormBranding, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, FromQueryResult,
    JoinType, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, Set,
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
        let form = Form::find_by_id(form_id.clone())
            .into_model::<APIFrontendForm>()
            .one(conn)
            .await?
            .ok_or(GetFormError::NotFound)?;

        let questions = QuestionManager::get_all_for_form(conn, form_id.clone())
            .await
            .map_err(|e| match e {
                super::questions::GetQuestionError::DecodeError(e) => GetFormError::DecodeError(e),
                super::questions::GetQuestionError::DBError(e) => GetFormError::DBError(e),
            })?;

        let groups = QuestionGroupManager::list_all_for_form(conn, form_id)
            .await
            .map_err(|e| GetFormError::DBError(e))?;

        let mut branding: Option<APIFormBranding> = None;
        if let Some(branding_id) = &form.branding_id {
            branding = Some(
                FormBrandingManager::get_by_id(conn, branding_id.clone())
                    .await
                    .map_err(|e| GetFormError::DBError(e))?
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

            Submission::delete_many()
                .filter(
                    Condition::all()
                        .add(submission::Column::FormId.eq(form_id))
                        .add(submission::Column::CreatedAt.lt(cut_off_date)),
                )
                .exec(conn)
                .await?;
        }

        Ok(())
    }
}
