use sea_orm_migration::{prelude::*, sea_orm::EnumIter, sea_query::extension::postgres::Type};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_type(
                Type::alter()
                    .name(AuditLogTargetResourceEnum)
                    .add_value(AuditLogTargetResourceVariants::AuthSession),
            )
            .await
    }

    async fn down(&self, _: &SchemaManager) -> Result<(), DbErr> {
        println!("Warning: not dropping enum values for AuditLogTargetResourceEnum. See here if you really need to https://stackoverflow.com/a/47305844.");
        Ok(())
    }
}

#[derive(DeriveIden)]
struct AuditLogTargetResourceEnum;
#[derive(DeriveIden, EnumIter)]
enum AuditLogTargetResourceVariants {
    AuthSession,
}
