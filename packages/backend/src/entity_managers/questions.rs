use palform_client_common::form_management::question_types::{
    APIQuestion, APIQuestionConfiguration,
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
    DBError(#[from] DbErr),
    #[error("Decode config: {0}")]
    DecodeError(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum SetQuestionError {
    #[error("Database: {0}")]
    DBError(#[from] DbErr),
    #[error("Encode config: {0}")]
    EncodeError(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum CreateQuestionError {
    #[error("Database: {0}")]
    DBError(#[from] DbErr),
    #[error("Encode config: {0}")]
    EncodeError(#[from] serde_json::Error),
    #[error("Unrecognised type for question: {0}")]
    CreateDefault(String),
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

    pub async fn check_question_exist<T: ConnectionTrait>(
        conn: &T,
        question_id: PalformDatabaseID<IDQuestion>,
    ) -> Result<bool, DbErr> {
        Question::find_by_id(question_id)
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn verify_question_group<T: ConnectionTrait>(
        conn: &T,
        question_id: PalformDatabaseID<IDQuestion>,
        question_group_id: PalformDatabaseID<IDQuestionGroup>,
    ) -> Result<bool, DbErr> {
        Question::find_by_id(question_id)
            .filter(question::Column::GroupId.eq(question_group_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn create_default_question<T: ConnectionTrait>(
        conn: &T,
        group_id: PalformDatabaseID<IDQuestionGroup>,
        question_type: String,
        position: i32,
    ) -> Result<APIQuestion, CreateQuestionError> {
        let default_config = APIQuestionConfiguration::default_for_type(question_type.clone())
            .map_err(|_| CreateQuestionError::CreateDefault(question_type))?;
        let encoded_config =
            QuestionWithEncodedConfiguration::encode_config(default_config.clone())?;
        let new_id = PalformDatabaseID::<IDQuestion>::random();
        let new_question = question::ActiveModel {
            id: Set(new_id),
            title: Set("Untitled".to_string()),
            description: Set(None),
            required: Set(false),
            configuration: Set(encoded_config),
            position: Set(position),
            group_id: Set(group_id),
            ..Default::default()
        };

        let new_question = new_question.insert(conn).await?;
        let new_question = APIQuestion {
            id: new_id,
            title: new_question.title,
            description: new_question.description,
            required: new_question.required,
            configuration: default_config,
            position,
            group_id,
        };
        Ok(new_question)
    }

    pub async fn set_question<T: ConnectionTrait>(
        conn: &T,
        group_id: PalformDatabaseID<IDQuestionGroup>,
        question: APIQuestion,
    ) -> Result<APIQuestion, SetQuestionError> {
        let encoded_config =
            QuestionWithEncodedConfiguration::encode_config(question.configuration)?;
        let updated_question = question::ActiveModel {
            id: Set(question.id),
            title: Set(question.title),
            description: Set(question.description),
            required: Set(question.required),
            configuration: Set(encoded_config),
            position: Set(question.position),
            group_id: Set(group_id),
            ..Default::default()
        };

        let question = updated_question.update(conn).await?;
        APIQuestion::try_from(QuestionWithEncodedConfiguration::from(question))
            .map_err(|e| SetQuestionError::EncodeError(e))
    }

    pub async fn delete_question<T: ConnectionTrait>(
        conn: &T,
        question_id: PalformDatabaseID<IDQuestion>,
    ) -> Result<(), DbErr> {
        Question::delete_by_id(question_id)
            .exec(conn)
            .await
            .map(|_| ())
    }
}
