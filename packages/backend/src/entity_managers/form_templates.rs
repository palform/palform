use palform_entities::{
    form, form_template, form_template_category, form_template_category_assignment, prelude::*,
    team,
};
use palform_migration::{Expr, SimpleExpr};
use palform_tsid::{
    resources::{IDForm, IDFormTemplateCategory},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, QueryFilter, QueryOrder,
    QuerySelect, RelationTrait,
};

use crate::api_entities::form_template::{APIFormTemplate, APIFormTemplateCategory};

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
}
