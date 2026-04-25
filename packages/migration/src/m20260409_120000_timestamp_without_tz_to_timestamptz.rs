use sea_orm_migration::prelude::*;
use sea_query::{Alias, Iden};

#[derive(DeriveMigrationName)]
pub struct Migration;

fn timestamptz_from_naive_utc<C: Iden + 'static>(col: C) -> ColumnDef {
    let name = col.to_string();
    let mut def = ColumnDef::new(col);
    def.custom(Alias::new(format!(
        r#"timestamptz USING ("{}" AT TIME ZONE 'UTC')"#,
        name
    )));
    def
}

fn timestamp_from_timestamptz_utc<C: Iden + 'static>(col: C) -> ColumnDef {
    let name = col.to_string();
    let mut def = ColumnDef::new(col);
    def.custom(Alias::new(format!(
        r#"timestamp USING ("{}" AT TIME ZONE 'UTC')"#,
        name
    )));
    def
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(AdminPublicKey::Table)
                    .modify_column(timestamptz_from_naive_utc(AdminPublicKey::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminPublicKey::Table)
                    .modify_column(timestamptz_from_naive_utc(AdminPublicKey::ExpiresAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUser::Table)
                    .modify_column(timestamptz_from_naive_utc(AdminUser::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserEmailVerification::Table)
                    .modify_column(timestamptz_from_naive_utc(
                        AdminUserEmailVerification::CreatedAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserEmailVerification::Table)
                    .modify_column(timestamptz_from_naive_utc(
                        AdminUserEmailVerification::ExpiresAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactor::Table)
                    .modify_column(timestamptz_from_naive_utc(
                        AdminUserSecondAuthenticationFactor::CreatedAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactorSession::Table)
                    .modify_column(timestamptz_from_naive_utc(
                        AdminUserSecondAuthenticationFactorSession::CreatedAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactorSession::Table)
                    .modify_column(timestamptz_from_naive_utc(
                        AdminUserSecondAuthenticationFactorSession::ExpiresAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AuditLogEntry::Table)
                    .modify_column(timestamptz_from_naive_utc(AuditLogEntry::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AuthToken::Table)
                    .modify_column(timestamptz_from_naive_utc(AuthToken::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AuthToken::Table)
                    .modify_column(timestamptz_from_naive_utc(AuthToken::ExpiresAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(DeletedSubmission::Table)
                    .modify_column(timestamptz_from_naive_utc(DeletedSubmission::DeletedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FeedbackItem::Table)
                    .modify_column(timestamptz_from_naive_utc(FeedbackItem::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FillAccessToken::Table)
                    .modify_column(timestamptz_from_naive_utc(FillAccessToken::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FillAccessToken::Table)
                    .modify_column(timestamptz_from_naive_utc(FillAccessToken::ExpiresAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Form::Table)
                    .modify_column(timestamptz_from_naive_utc(Form::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Organisation::Table)
                    .modify_column(timestamptz_from_naive_utc(Organisation::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Organisation::Table)
                    .modify_column(timestamptz_from_naive_utc(
                        Organisation::BillingSubmissionBlock,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationInvite::Table)
                    .modify_column(timestamptz_from_naive_utc(OrganisationInvite::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationInvite::Table)
                    .modify_column(timestamptz_from_naive_utc(OrganisationInvite::ExpiresAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(SocialAuthConnection::Table)
                    .modify_column(timestamptz_from_naive_utc(SocialAuthConnection::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Submission::Table)
                    .modify_column(timestamptz_from_naive_utc(Submission::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TeamAsset::Table)
                    .modify_column(timestamptz_from_naive_utc(TeamAsset::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Webhook::Table)
                    .modify_column(timestamptz_from_naive_utc(Webhook::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(WebhookJob::Table)
                    .modify_column(timestamptz_from_naive_utc(WebhookJob::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(WebhookJob::Table)
                    .modify_column(timestamptz_from_naive_utc(WebhookJob::DoneAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(WebhookJob::Table)
                    .modify_column(timestamp_from_timestamptz_utc(WebhookJob::DoneAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(WebhookJob::Table)
                    .modify_column(timestamp_from_timestamptz_utc(WebhookJob::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Webhook::Table)
                    .modify_column(timestamp_from_timestamptz_utc(Webhook::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(TeamAsset::Table)
                    .modify_column(timestamp_from_timestamptz_utc(TeamAsset::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Submission::Table)
                    .modify_column(timestamp_from_timestamptz_utc(Submission::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(SocialAuthConnection::Table)
                    .modify_column(timestamp_from_timestamptz_utc(
                        SocialAuthConnection::CreatedAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationInvite::Table)
                    .modify_column(timestamp_from_timestamptz_utc(
                        OrganisationInvite::ExpiresAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(OrganisationInvite::Table)
                    .modify_column(timestamp_from_timestamptz_utc(
                        OrganisationInvite::CreatedAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Organisation::Table)
                    .modify_column(timestamp_from_timestamptz_utc(
                        Organisation::BillingSubmissionBlock,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Organisation::Table)
                    .modify_column(timestamp_from_timestamptz_utc(Organisation::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Form::Table)
                    .modify_column(timestamp_from_timestamptz_utc(Form::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FillAccessToken::Table)
                    .modify_column(timestamp_from_timestamptz_utc(FillAccessToken::ExpiresAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FillAccessToken::Table)
                    .modify_column(timestamp_from_timestamptz_utc(FillAccessToken::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(FeedbackItem::Table)
                    .modify_column(timestamp_from_timestamptz_utc(FeedbackItem::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(DeletedSubmission::Table)
                    .modify_column(timestamp_from_timestamptz_utc(DeletedSubmission::DeletedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AuthToken::Table)
                    .modify_column(timestamp_from_timestamptz_utc(AuthToken::ExpiresAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AuthToken::Table)
                    .modify_column(timestamp_from_timestamptz_utc(AuthToken::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AuditLogEntry::Table)
                    .modify_column(timestamp_from_timestamptz_utc(AuditLogEntry::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactorSession::Table)
                    .modify_column(timestamp_from_timestamptz_utc(
                        AdminUserSecondAuthenticationFactorSession::ExpiresAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactorSession::Table)
                    .modify_column(timestamp_from_timestamptz_utc(
                        AdminUserSecondAuthenticationFactorSession::CreatedAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserSecondAuthenticationFactor::Table)
                    .modify_column(timestamp_from_timestamptz_utc(
                        AdminUserSecondAuthenticationFactor::CreatedAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserEmailVerification::Table)
                    .modify_column(timestamp_from_timestamptz_utc(
                        AdminUserEmailVerification::ExpiresAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUserEmailVerification::Table)
                    .modify_column(timestamp_from_timestamptz_utc(
                        AdminUserEmailVerification::CreatedAt,
                    ))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminUser::Table)
                    .modify_column(timestamp_from_timestamptz_utc(AdminUser::CreatedAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminPublicKey::Table)
                    .modify_column(timestamp_from_timestamptz_utc(AdminPublicKey::ExpiresAt))
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(AdminPublicKey::Table)
                    .modify_column(timestamp_from_timestamptz_utc(AdminPublicKey::CreatedAt))
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum AdminPublicKey {
    Table,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum AdminUser {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum AdminUserEmailVerification {
    Table,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum AdminUserSecondAuthenticationFactor {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum AdminUserSecondAuthenticationFactorSession {
    Table,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum AuditLogEntry {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum AuthToken {
    Table,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum DeletedSubmission {
    Table,
    DeletedAt,
}

#[derive(DeriveIden)]
enum FeedbackItem {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum FillAccessToken {
    Table,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum Form {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Organisation {
    Table,
    CreatedAt,
    BillingSubmissionBlock,
}

#[derive(DeriveIden)]
enum OrganisationInvite {
    Table,
    CreatedAt,
    ExpiresAt,
}

#[derive(DeriveIden)]
enum SocialAuthConnection {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Submission {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum TeamAsset {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Webhook {
    Table,
    CreatedAt,
}

#[derive(DeriveIden)]
enum WebhookJob {
    Table,
    CreatedAt,
    DoneAt,
}
