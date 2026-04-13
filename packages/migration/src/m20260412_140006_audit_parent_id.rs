use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AuditLogEntry::Table)
                    .add_column(
                        ColumnDef::new(AuditLogEntry::TargetResourceParentIds)
                            .array(ColumnType::String(StringLen::None))
                            .not_null()
                            .default(Value::Array(ArrayType::String, Some(Box::default()))),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AuditLogEntry::Table)
                    .drop_column(AuditLogEntry::TargetResourceParentIds)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AuditLogEntry {
    Table,
    TargetResourceParentIds,
}
