use palform_client_common::form_management::question_group::APIQuestionGroup;
use palform_entities::{prelude::*, question_group};
use palform_tsid::{
    resources::{IDForm, IDQuestionGroup},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};

use crate::api_entities::question_group::QuestionGroupWithEncodedStrategy;

pub struct QuestionGroupManager;

impl QuestionGroupManager {
    pub async fn verify_question_group_form<T: ConnectionTrait>(
        conn: &T,
        question_group_id: PalformDatabaseID<IDQuestionGroup>,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<bool, DbErr> {
        QuestionGroup::find_by_id(question_group_id)
            .filter(question_group::Column::FormId.eq(form_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    pub async fn list_all_for_form<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<Vec<APIQuestionGroup>, DbErr> {
        let resp = QuestionGroup::find()
            .filter(question_group::Column::FormId.eq(form_id))
            .order_by_asc(question_group::Column::Position)
            .into_model::<QuestionGroupWithEncodedStrategy>()
            .all(conn)
            .await?;

        let resp: Result<Vec<APIQuestionGroup>, serde_json::Error> = resp
            .iter()
            .map(|qg| APIQuestionGroup::try_from(qg.to_owned()))
            .collect();

        let resp = resp.map_err(|e| DbErr::TryIntoErr {
            from: "QuestionGroupWithEncodedStrategy",
            into: "APIQuestionGroup",
            source: e.into(),
        })?;

        Ok(resp)
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        position: i32,
        title: Option<String>,
        description: Option<String>,
    ) -> Result<PalformDatabaseID<IDQuestionGroup>, DbErr> {
        let new_group = question_group::ActiveModel {
            id: Set(PalformDatabaseID::<IDQuestionGroup>::random()),
            position: Set(position),
            title: Set(title),
            description: Set(description),
            form_id: Set(form_id),
            ..Default::default()
        };
        let resp = new_group.insert(conn).await?;
        Ok(resp.id)
    }

    pub async fn update<T: ConnectionTrait>(conn: &T, data: APIQuestionGroup) -> Result<(), DbErr> {
        let strategy = serde_json::to_value(data.step_strategy).map_err(|e| DbErr::TryIntoErr {
            from: "APIQuestionGroup",
            into: "Json",
            source: e.into(),
        })?;

        let updated_group = question_group::ActiveModel {
            id: Set(data.id),
            position: Set(data.position),
            title: Set(data.title),
            description: Set(data.description),
            step_strategy: Set(strategy),
            ..Default::default()
        };
        updated_group.update(conn).await?;
        Ok(())
    }

    pub async fn delete<T: ConnectionTrait>(
        conn: &T,
        id: PalformDatabaseID<IDQuestionGroup>,
    ) -> Result<(), DbErr> {
        QuestionGroup::delete_by_id(id).exec(conn).await.map(|_| ())
    }
}
