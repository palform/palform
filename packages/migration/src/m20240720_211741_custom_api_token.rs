use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminUser::Table)
                    .add_column(
                        ColumnDef::new(AdminUser::ServiceAccountAuthToken)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminUser::Table)
                    .drop_column(AdminUser::ServiceAccountAuthToken)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    ServiceAccountAuthToken,
}
