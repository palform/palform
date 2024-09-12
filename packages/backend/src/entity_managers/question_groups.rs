use palform_client_common::form_management::question_group::APIQuestionGroup;
use palform_entities::{prelude::*, question_group};
use palform_tsid::{
    resources::{IDForm, IDQuestionGroup},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
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
}
