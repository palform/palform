use std::collections::HashMap;

use palform_client_common::form_management::question_group::{
    APIQuestionGroupStepStrategy, APIQuestionGroupStepStrategyJumpCase,
    APIQuestionGroupStepStrategyJumpCaseCondition,
    APIQuestionGroupStepStrategyJumpCaseConditionList,
};
use palform_entities::{
    form, form_template, form_template_category, form_template_category_assignment, prelude::*,
    question, question_group, team,
};
use palform_migration::{Expr, SimpleExpr};
use palform_tsid::{
    resources::{IDForm, IDFormTemplateCategory, IDQuestion, IDQuestionGroup, IDTeam},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};

use crate::api_entities::{
    form::APIForm,
    form_template::{APIFormTemplate, APIFormTemplateCategory},
};

use super::forms::FormManager;

pub struct FormTemplatesManager;

impl FormTemplatesManager {
    pub async fn list_categories<T: ConnectionTrait>(
        conn: &T,
    ) -> Result<Vec<APIFormTemplateCategory>, DbErr> {
        FormTemplateCategory::find()
            .order_by_asc(form_template_category::Column::Name)
            .join(
                JoinType::LeftJoin,
                form_template_category::Relation::FormTemplateCategoryAssignment.def(),
            )
            .column_as(
                form_template_category_assignment::Column::FormId.count(),
                "template_count",
            )
            .group_by(form_template_category::Column::Id)
            .order_by_desc(SimpleExpr::Custom("template_count".to_string()))
            .into_model()
            .all(conn)
            .await
    }

    pub async fn get_category<T: ConnectionTrait>(
        conn: &T,
        category_id: PalformDatabaseID<IDFormTemplateCategory>,
    ) -> Result<Option<APIFormTemplateCategory>, DbErr> {
        FormTemplateCategory::find_by_id(category_id)
            .join(
                JoinType::LeftJoin,
                form_template_category::Relation::FormTemplateCategoryAssignment.def(),
            )
            .column_as(
                form_template_category_assignment::Column::FormId.count(),
                "template_count",
            )
            .group_by(form_template_category::Column::Id)
            .into_model()
            .one(conn)
            .await
    }

    pub async fn list_in_category<T: ConnectionTrait>(
        conn: &T,
        category_id: PalformDatabaseID<IDFormTemplateCategory>,
    ) -> Result<Vec<APIFormTemplate>, DbErr> {
        FormTemplate::find()
            .join(
                JoinType::InnerJoin,
                form_template::Relation::FormTemplateCategoryAssignment.def(),
            )
            .join(JoinType::InnerJoin, form_template::Relation::Form.def())
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .column_as(form::Column::EditorName, "name")
            .column_as(form::Column::Id, "id")
            .column_as(team::Column::OrganisationId, "organisation_id")
            .order_by_desc(form_template::Column::Views)
            .filter(form_template_category_assignment::Column::CategoryId.eq(category_id))
            .into_model()
            .all(conn)
            .await
    }

    pub async fn list_top_across_categories<T: ConnectionTrait>(
        conn: &T,
        top_n: u64,
    ) -> Result<Vec<APIFormTemplate>, DbErr> {
        FormTemplate::find()
            .join(JoinType::InnerJoin, form_template::Relation::Form.def())
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .column_as(form::Column::EditorName, "name")
            .column_as(form::Column::Id, "id")
            .column_as(team::Column::OrganisationId, "organisation_id")
            .order_by_desc(form_template::Column::Views)
            .limit(top_n)
            .into_model()
            .all(conn)
            .await
    }

    pub async fn get<T: ConnectionTrait>(
        conn: &T,
        template_id: PalformDatabaseID<IDForm>,
    ) -> Result<Option<APIFormTemplate>, DbErr> {
        FormTemplate::find_by_id(template_id)
            .join(JoinType::InnerJoin, form_template::Relation::Form.def())
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .column_as(form::Column::EditorName, "name")
            .column_as(form::Column::Id, "id")
            .column_as(team::Column::OrganisationId, "organisation_id")
            .into_model()
            .one(conn)
            .await
    }

    pub async fn report_view<T: ConnectionTrait>(
        conn: &T,
        template_id: PalformDatabaseID<IDForm>,
    ) -> Result<(), DbErr> {
        FormTemplate::update_many()
            .col_expr(
                form_template::Column::Views,
                Expr::col(form_template::Column::Views).add(1),
            )
            .filter(form_template::Column::FormId.eq(template_id))
            .exec(conn)
            .await?;
        Ok(())
    }

    async fn report_clone<T: ConnectionTrait>(
        conn: &T,
        template_id: PalformDatabaseID<IDForm>,
    ) -> Result<(), DbErr> {
        FormTemplate::update_many()
            .col_expr(
                form_template::Column::Clones,
                Expr::col(form_template::Column::Clones).add(1),
            )
            .filter(form_template::Column::FormId.eq(template_id))
            .exec(conn)
            .await?;
        Ok(())
    }

    pub async fn clone<T: ConnectionTrait>(
        conn: &T,
        template_id: PalformDatabaseID<IDForm>,
        into_team: PalformDatabaseID<IDTeam>,
    ) -> Result<APIForm, DbErr> {
        let count = FormTemplate::find_by_id(template_id).count(conn).await?;
        if count != 1 {
            return Err(DbErr::RecordNotFound(
                "No corresponding template found".to_string(),
            ));
        }

        let template_form = Form::find_by_id(template_id)
            .one(conn)
            .await?
            .ok_or(DbErr::RecordNotFound("Form not found".to_string()))?;

        let template_qgs = QuestionGroup::find()
            .filter(question_group::Column::FormId.eq(template_form.id))
            .all(conn)
            .await?;

        let template_questions = Question::find()
            .join(JoinType::InnerJoin, question::Relation::QuestionGroup.def())
            .filter(question_group::Column::FormId.eq(template_form.id))
            .all(conn)
            .await?;

        let new_form_id = PalformDatabaseID::<IDForm>::random();
        let new_form = form::ActiveModel {
            id: Set(new_form_id),
            editor_name: Set(template_form.editor_name),
            title: Set(template_form.title),
            team_id: Set(into_team),
            end_configuration: Set(template_form.end_configuration),
            one_question_per_page: Set(template_form.one_question_per_page),
            ..Default::default()
        };
        new_form.insert(conn).await?;

        let mut old_to_new_question_groups =
            HashMap::<PalformDatabaseID<IDQuestionGroup>, PalformDatabaseID<IDQuestionGroup>>::new(
            );

        for question_group in &template_qgs {
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
        for question in template_questions {
            let new_group_id = old_to_new_question_groups
                .get(&question.group_id)
                .ok_or(DbErr::RecordNotFound("This cannot happen".to_string()))?;

            let new_question_id = PalformDatabaseID::<IDQuestion>::random();
            let new_question = question::ActiveModel {
                id: Set(new_question_id),
                group_id: Set(new_group_id.clone()),
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

        for question_group in &template_qgs {
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

        Self::report_clone(conn, template_id).await?;
        Ok(newly_created_form)
    }
}
