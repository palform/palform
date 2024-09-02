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
            .create_type(
                Type::create()
                    .as_enum(AuditLogTargetResourceEnum)
                    .values(AuditLogTargetResourceVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_type(
                Type::create()
                    .as_enum(AuditLogVerbEnum)
                    .values(AuditLogVerbVariants::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(AuditLogEntry::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AuditLogEntry::Id)
                            .uuid()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(AuditLogEntry::OrganisationId)
                            .uuid()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_audit_log_org")
                            .from(AuditLogEntry::Table, AuditLogEntry::OrganisationId)
                            .to(Organisation::Table, Organisation::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(AuditLogEntry::UserId).uuid().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_audit_log_user")
                            .from(AuditLogEntry::Table, AuditLogEntry::UserId)
                            .to(AdminUser::Table, AdminUser::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(AuditLogEntry::TargetResourceType)
                            .enumeration(
                                AuditLogTargetResourceEnum,
                                AuditLogTargetResourceVariants::iter(),
                            )
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(AuditLogEntry::TargetResourceId)
                            .uuid()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(AuditLogEntry::Verb)
                            .enumeration(AuditLogVerbEnum, AuditLogVerbVariants::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(AuditLogEntry::Note).string().null())
                    .col(
                        ColumnDef::new(AuditLogEntry::CreatedAt)
                            .timestamp()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AuditLogEntry::Table).to_owned())
            .await?;

        manager
            .drop_type(Type::drop().name(AuditLogTargetResourceEnum).to_owned())
            .await?;
        manager
            .drop_type(Type::drop().name(AuditLogVerbEnum).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum Organisation {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum AuditLogEntry {
    Table,
    Id,
    OrganisationId,
    UserId,
    TargetResourceType,
    TargetResourceId,
    Verb,
    Note,
    CreatedAt,
}

#[derive(DeriveIden)]
struct AuditLogTargetResourceEnum;
#[derive(DeriveIden, EnumIter)]
enum AuditLogTargetResourceVariants {
    OrganisationMember,
    Team,
    TeamMember,
    Form,
    Branding,
}

#[derive(DeriveIden)]
struct AuditLogVerbEnum;
#[derive(DeriveIden, EnumIter)]
enum AuditLogVerbVariants {
    Create,
    Read,
    Update,
    Delete,
}
