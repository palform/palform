use sea_orm_migration::{
    prelude::*,
    sea_orm::{EnumIter, Iterable},
    sea_query::extension::postgres::Type,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Organisation::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Organisation::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Organisation::DisplayName)
                            .string()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AdminUser::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AdminUser::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AdminUser::DisplayName).string().not_null())
                    .col(ColumnDef::new(AdminUser::Email).string().not_null())
                    .col(
                        ColumnDef::new(AdminUser::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuthToken::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuthToken::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AuthToken::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_token_user")
                            .from(AuthToken::Table, AuthToken::UserId)
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(AuthToken::Hash).string().not_null())
                    .col(
                        ColumnDef::new(AuthToken::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(AuthToken::ExpiresAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(OrganisationMemberRoleEnum)
                    .values(OrganisationMemberRoleVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(OrganisationMembership::Table)
                    .primary_key(
                        Index::create()
                            .col(OrganisationMembership::OrganisationId)
                            .col(OrganisationMembership::UserId),
                    )
                    .col(
                        ColumnDef::new(OrganisationMembership::OrganisationId)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(OrganisationMembership::UserId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orgmember_organisation")
                            .from(
                                OrganisationMembership::Table,
                                OrganisationMembership::OrganisationId,
                            )
                            .to(Organisation::Table, Organisation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_orgmember_user")
                            .from(
                                OrganisationMembership::Table,
                                OrganisationMembership::UserId,
                            )
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(OrganisationMembership::Role)
                            .enumeration(
                                OrganisationMemberRoleEnum,
                                OrganisationMemberRoleVariants::iter(),
                            )
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Form::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Form::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Form::OwnerOrganisationId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_form_owner")
                            .from(Form::Table, Form::OwnerOrganisationId)
                            .to(Organisation::Table, Organisation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Form::EditorName).string().not_null())
                    .col(ColumnDef::new(Form::Title).string().null())
                    .col(
                        ColumnDef::new(Form::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Question::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Question::Id).uuid().not_null().primary_key())
                    .col(ColumnDef::new(Question::FormId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_question_form")
                            .from(Question::Table, Question::FormId)
                            .to(Form::Table, Form::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Question::Title).string().not_null())
                    .col(ColumnDef::new(Question::Description).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Submission::Table)
                    .col(
                        ColumnDef::new(Submission::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Submission::FormId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_submission_form")
                            .from(Submission::Table, Submission::FormId)
                            .to(Form::Table, Form::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(Submission::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(Submission::EncryptedData)
                            .binary()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Submission::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Question::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Form::Table).to_owned())
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(OrganisationMembership::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_type(Type::drop().name(OrganisationMemberRoleEnum).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(AdminUser::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Organisation::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Organisation {
    Table,
    Id,
    DisplayName,
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    Id,
    DisplayName,
    Email,
    CreatedAt,
}

#[derive(DeriveIden)]
enum AuthToken {
    Table,
    Id,
    UserId,
    Hash,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
struct OrganisationMemberRoleEnum;
#[derive(DeriveIden, EnumIter)]
enum OrganisationMemberRoleVariants {
    Viewer,
    Editor,
    Admin,
}

#[derive(DeriveIden)]
enum OrganisationMembership {
    Table,
    OrganisationId,
    UserId,
    Role,
}

#[derive(DeriveIden)]
enum Form {
    Table,
    Id,
    OwnerOrganisationId,
    EditorName,
    Title,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Question {
    Table,
    Id,
    FormId,
    Title,
    Description,
}

#[derive(DeriveIden)]
enum Submission {
    Table,
    Id,
    FormId,
    CreatedAt,
    EncryptedData,
}
