use std::collections::HashSet;

use palform_client_common::form_management::{
    question_group::APIQuestionGroup, question_types::APIQuestion,
};
use palform_entities::{prelude::*, question, question_group};
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDQuestion, IDQuestionGroup, IDUnknown},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};
use thiserror::Error;

use crate::api_entities::question::QuestionWithEncodedConfiguration;

use super::billing_entitlement_proxy::BillingEntitlementContextualCountTrait;

#[derive(Debug, Error)]
pub enum GetQuestionError {
    #[error("Database: {0}")]
    DB(#[from] DbErr),
    #[error("Decode config: {0}")]
    Decode(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum SetQuestionError {
    #[error("Database: {0}")]
    DB(#[from] DbErr),
    #[error("Encode config: {0}")]
    Encode(#[from] serde_json::Error),
    #[error("{0}")]
    Validation(String),
}

pub struct QuestionManager;

impl BillingEntitlementContextualCountTrait for QuestionManager {
    async fn billing_count<T: ConnectionTrait>(
        conn: &T,
        _org_id: PalformDatabaseID<IDOrganisation>,
        context_resource_id: PalformDatabaseID<IDUnknown>,
    ) -> Result<u64, DbErr> {
        Question::find()
            .join(JoinType::InnerJoin, question::Relation::QuestionGroup.def())
            .filter(question_group::Column::FormId.eq(context_resource_id))
            .count(conn)
            .await
    }
}

impl QuestionManager {
    pub async fn get_all_for_form<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<Vec<APIQuestion>, GetQuestionError> {
        let raw_questions = Question::find()
            .join(JoinType::InnerJoin, question::Relation::QuestionGroup.def())
            .filter(question_group::Column::FormId.eq(form_id))
            .order_by_asc(question::Column::Position)
            .into_model::<QuestionWithEncodedConfiguration>()
            .all(conn)
            .await?;

        let api_questions: Result<Vec<APIQuestion>, serde_json::Error> = raw_questions
            .iter()
            .map(|question| APIQuestion::try_from(question.clone()))
            .collect();

        let api_questions = api_questions?;
        Ok(api_questions)
    }

    pub async fn get_by_id<T: ConnectionTrait>(
        conn: &T,
        question_id: PalformDatabaseID<IDQuestion>,
    ) -> Result<Option<QuestionWithEncodedConfiguration>, DbErr> {
        Question::find_by_id(question_id)
            .into_model::<QuestionWithEncodedConfiguration>()
            .one(conn)
            .await
    }

    pub fn validate_question_internal_name(internal_name: String) -> bool {
        if internal_name.is_empty() {
            return false;
        }

        if internal_name
            .chars()
            .next()
            .is_some_and(|v| v > '0' && v < '9')
        {
            return false;
        }

        for c in internal_name.chars() {
            if !c.is_ascii_lowercase() && !c.is_ascii_uppercase() && !c.is_ascii_digit() && c != '_'
            {
                return false;
            }
        }

        true
    }

    pub async fn save_questions_and_groups<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        groups: Vec<APIQuestionGroup>,
        questions: Vec<APIQuestion>,
    ) -> Result<(), SetQuestionError> {
        let current_question_ids: Vec<PalformDatabaseID<IDQuestion>> = Question::find()
            .select_only()
            .column(question::Column::Id)
            .join(JoinType::InnerJoin, question::Relation::QuestionGroup.def())
            .filter(question_group::Column::FormId.eq(form_id))
            .into_tuple()
            .all(conn)
            .await?;
        let current_question_ids =
            HashSet::<PalformDatabaseID<IDQuestion>>::from_iter(current_question_ids);

        let current_group_ids: Vec<PalformDatabaseID<IDQuestionGroup>> = QuestionGroup::find()
            .select_only()
            .column(question_group::Column::Id)
            .filter(question_group::Column::FormId.eq(form_id))
            .into_tuple()
            .all(conn)
            .await?;
        let current_group_ids =
            HashSet::<PalformDatabaseID<IDQuestionGroup>>::from_iter(current_group_ids);

        let encoded_groups: Result<
            Vec<(
                PalformDatabaseID<IDQuestionGroup>,
                question_group::ActiveModel,
            )>,
            SetQuestionError,
        > = groups
            .iter()
            .enumerate()
            .map(|(i, g)| {
                let step_strategy = serde_json::to_value(g.step_strategy.clone())
                    .map_err(SetQuestionError::Encode)?;

                Ok((
                    g.id,
                    question_group::ActiveModel {
                        id: Set(g.id),
                        position: Set(i as i32),
                        title: Set(g.title.clone()),
                        description: Set(g.description.clone()),
                        step_strategy: Set(step_strategy),
                        form_id: Set(form_id),
                    },
                ))
            })
            .collect();
        let encoded_groups = encoded_groups?;

        let encoded_questions: Result<
            Vec<(PalformDatabaseID<IDQuestion>, question::ActiveModel)>,
            SetQuestionError,
        > = questions
            .iter()
            .enumerate()
            .map(|(i, q)| {
                let configuration =
                    QuestionWithEncodedConfiguration::encode_config(q.configuration.clone())
                        .map_err(SetQuestionError::Encode)?;

                if let Some(internal_name) = q.internal_name.clone() {
                    if !Self::validate_question_internal_name(internal_name) {
                        return Err(SetQuestionError::Validation(
                            "invalid internal name".to_string(),
                        ));
                    }
                }

                Ok((
                    q.id,
                    question::ActiveModel {
                        id: Set(q.id),
                        title: Set(q.title.clone()),
                        internal_name: Set(q.internal_name.clone()),
                        description: Set(q.description.clone()),
                        required: Set(q.required),
                        configuration: Set(configuration),
                        position: Set(i as i32),
                        group_id: Set(q.group_id),
                    },
                ))
            })
            .collect();
        let encoded_questions = encoded_questions?;

        let mut seen_group_ids = HashSet::<PalformDatabaseID<IDQuestionGroup>>::new();
        for (group_id, group) in encoded_groups {
            seen_group_ids.insert(group_id);
            if current_group_ids.contains(&group_id) {
                group.update(conn).await?;
            } else {
                group.insert(conn).await?;
            }
        }

        let deleted_group_ids: Vec<PalformDatabaseID<IDQuestionGroup>> = current_group_ids
            .difference(&seen_group_ids)
            .cloned()
            .collect();
        if !deleted_group_ids.is_empty() {
            QuestionGroup::delete_many()
                .filter(question_group::Column::Id.is_in(deleted_group_ids))
                .exec(conn)
                .await?;
        }

        let mut seen_question_ids = HashSet::<PalformDatabaseID<IDQuestion>>::new();
        for (question_id, question) in encoded_questions {
            seen_question_ids.insert(question_id);
            if current_question_ids.contains(&question_id) {
                question.update(conn).await?;
            } else {
                question.insert(conn).await?;
            }
        }

        let deleted_question_ids: Vec<PalformDatabaseID<IDQuestion>> = current_question_ids
            .difference(&seen_question_ids)
            .cloned()
            .collect();
        if !deleted_question_ids.is_empty() {
            Question::delete_many()
                .filter(question::Column::Id.is_in(deleted_question_ids))
                .exec(conn)
                .await?;
        }

        Ok(())
    }
}
