use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Organisation::Table)
                    .add_column(
                        ColumnDef::new(Organisation::BillingAllowOverage)
                            .boolean()
                            .not_null()
                            .default(false),
                    )
                    .drop_column(Organisation::BillingSubscriptionId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Organisation::Table)
                    .drop_column(Organisation::BillingAllowOverage)
                    .add_column(
                        ColumnDef::new(Organisation::BillingSubscriptionId)
                            .string()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Organisation {
    Table,
    BillingAllowOverage,
    BillingSubscriptionId,
}
