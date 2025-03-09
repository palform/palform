use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FeedbackItem::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FeedbackItem::Id)
                            .big_unsigned()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FeedbackItem::Score).unsigned().not_null())
                    .col(ColumnDef::new(FeedbackItem::Comment).text().null())
                    .col(
                        ColumnDef::new(FeedbackItem::CreatedAt)
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
            .drop_table(Table::drop().table(FeedbackItem::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FeedbackItem {
    Table,
    Id,
    Score,
    Comment,
    CreatedAt,
}
