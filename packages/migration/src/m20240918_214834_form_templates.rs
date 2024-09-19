use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FormTemplate::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FormTemplate::FormId)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_template_form")
                            .from(FormTemplate::Table, FormTemplate::FormId)
                            .to(Form::Table, Form::Id),
                    )
                    .col(ColumnDef::new(FormTemplate::Description).string().null())
                    .col(
                        ColumnDef::new(FormTemplate::Views)
                            .big_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(
                        ColumnDef::new(FormTemplate::Clones)
                            .big_unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(FormTemplate::AuthorName).string().not_null())
                    .col(
                        ColumnDef::new(FormTemplate::PreviewToken)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_template_fill_access_token")
                            .from(FormTemplate::Table, FormTemplate::PreviewToken)
                            .to(FillAccessToken::Table, FillAccessToken::Id),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FormTemplateCategory::Table)
                    .col(
                        ColumnDef::new(FormTemplateCategory::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(FormTemplateCategory::Name)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(FormTemplateCategoryAssignment::Table)
                    .col(
                        ColumnDef::new(FormTemplateCategoryAssignment::FormId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_form_template_category_form")
                            .from(
                                FormTemplateCategoryAssignment::Table,
                                FormTemplateCategoryAssignment::FormId,
                            )
                            .to(FormTemplate::Table, FormTemplate::FormId)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(FormTemplateCategoryAssignment::CategoryId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_form_template_category_category")
                            .from(
                                FormTemplateCategoryAssignment::Table,
                                FormTemplateCategoryAssignment::CategoryId,
                            )
                            .to(FormTemplateCategory::Table, FormTemplateCategory::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .primary_key(
                        Index::create()
                            .col(FormTemplateCategoryAssignment::FormId)
                            .col(FormTemplateCategoryAssignment::CategoryId),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(FormTemplateCategoryAssignment::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(FormTemplateCategory::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(FormTemplate::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Form {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum FillAccessToken {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum FormTemplate {
    Table,
    FormId,
    Description,
    Views,
    Clones,
    AuthorName,
    PreviewToken,
}

#[derive(DeriveIden)]
enum FormTemplateCategory {
    Table,
    Id,
    Name,
}

#[derive(DeriveIden)]
enum FormTemplateCategoryAssignment {
    Table,
    FormId,
    CategoryId,
}
