use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Form::Table)
                    .drop_column(Form::NotificationWebhookURL)
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(Webhook::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Webhook::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Webhook::FormId).big_unsigned().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_webhook_form")
                            .from(Webhook::Table, Webhook::FormId)
                            .to(Form::Table, Form::Id)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .col(ColumnDef::new(Webhook::Endpoint).string().not_null())
                    .col(ColumnDef::new(Webhook::SigningSecret).string().not_null())
                    .col(
                        ColumnDef::new(Webhook::CreatedAt)
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
                    .table(WebhookJob::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(WebhookJob::Id)
                            .big_unsigned()
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(WebhookJob::WebhookId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_webhook_job_webhook")
                            .from(WebhookJob::Table, WebhookJob::WebhookId)
                            .to(Webhook::Table, Webhook::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(WebhookJob::SubmissionId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_webhook_job_submission")
                            .from(WebhookJob::Table, WebhookJob::SubmissionId)
                            .to(Submission::Table, Submission::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .col(
                        ColumnDef::new(WebhookJob::CreatedAt)
                            .timestamp()
                            .default(Expr::current_timestamp())
                            .not_null(),
                    )
                    .col(ColumnDef::new(WebhookJob::DoneAt).timestamp().null())
                    .col(ColumnDef::new(WebhookJob::Error).string().null())
                    .col(
                        ColumnDef::new(WebhookJob::Retries)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Form::Table)
                    .add_column(ColumnDef::new(Form::NotificationWebhookURL).string().null())
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(WebhookJob::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Webhook::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Webhook {
    Table,
    Id,
    FormId,
    Endpoint,
    SigningSecret,
    CreatedAt,
}

#[derive(DeriveIden)]
enum WebhookJob {
    Table,
    Id,
    WebhookId,
    SubmissionId,
    CreatedAt,
    DoneAt,
    Error,
    Retries,
}

#[derive(DeriveIden)]
enum Form {
    Table,
    Id,
    NotificationWebhookURL,
}

#[derive(DeriveIden)]
enum Submission {
    Table,
    Id,
}
