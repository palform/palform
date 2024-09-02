use chrono::{DateTime, NaiveDateTime, Utc};
use lettre::{
    address::AddressError,
    message::{header::ContentType, Mailbox},
};
use palform_entities::{
    admin_user, deleted_submission, form, prelude::*, submission, team, team_membership,
};
use palform_migration::all;
use palform_tsid::{
    resources::{IDAdminUser, IDFillAccessToken, IDForm, IDOrganisation, IDSubmission, IDTeam},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, FromQueryResult,
    JoinType, Order, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, RelationTrait, Set,
};
use serde::Serialize;
use thiserror::Error;

use crate::{
    api_entities::submission::{APISubmissionCountPerForm, APISubmissionWebhookPayload},
    mail::{
        client::PalformMailClient,
        headers::{MailgunHeader, MailgunTemplateNameHeader, MailgunVariableListHeader},
    },
    webhooks::{submission::SubmissionWebhookManager, webhook::send_webhook},
};

#[derive(Debug, Error)]
pub enum SubmissionNotificationError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[error("Sending email: {0}")]
    Email(String),
    #[error("Sending webhook: {0}")]
    Webhook(String),
}

pub struct SubmissionManager;

impl SubmissionManager {
    pub async fn create_submission<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        fill_token_id: PalformDatabaseID<IDFillAccessToken>,
        data: Vec<u8>,
    ) -> Result<PalformDatabaseID<IDSubmission>, DbErr> {
        let new_submission = submission::ActiveModel {
            id: Set(PalformDatabaseID::<IDSubmission>::random()),
            form_id: Set(form_id),
            encrypted_data: Set(data),
            for_token: Set(Some(fill_token_id)),
            ..Default::default()
        };
        let resp = new_submission.insert(conn).await?;
        Ok(resp.id)
    }

    pub async fn list_submissions<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        since_id: Option<PalformDatabaseID<IDSubmission>>,
    ) -> Result<Vec<submission::Model>, DbErr> {
        let mut condition = Condition::all().add(submission::Column::FormId.eq(form_id));

        if let Some(since_id) = since_id {
            let mut since: Option<NaiveDateTime> = Submission::find_by_id(since_id)
                .select_only()
                .column(submission::Column::CreatedAt)
                .into_tuple()
                .one(conn)
                .await?;

            if since.is_none() {
                since = DeletedSubmission::find_by_id(since_id)
                    .select_only()
                    .column(deleted_submission::Column::DeletedAt)
                    .into_tuple()
                    .one(conn)
                    .await?;
            }

            // if we can't find the `since_id` either as a current submission or a deleted one,
            // then just ignore the condition altogether. better being correct in this (rare) case than 
            // doing some clever performance optimisation.
            if let Some(since) = since {
                condition = condition.add(submission::Column::CreatedAt.gt(since))
            }
        }

        Submission::find()
            .filter(condition)
            .order_by(submission::Column::CreatedAt, Order::Asc)
            .all(conn)
            .await
    }

    pub async fn list_deleted_submissions<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<Vec<PalformDatabaseID<IDSubmission>>, DbErr> {
        DeletedSubmission::find()
            .filter(all![deleted_submission::Column::FormId.eq(form_id),])
            .select_only()
            .column(deleted_submission::Column::Id)
            .order_by(deleted_submission::Column::DeletedAt, Order::Asc)
            .into_tuple()
            .all(conn)
            .await
    }

    pub async fn total_submission_count_in_org<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        after: NaiveDateTime,
    ) -> Result<u64, DbErr> {
        Submission::find()
            .join(JoinType::InnerJoin, submission::Relation::Form.def())
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .filter(all![
                team::Column::OrganisationId.eq(org_id),
                submission::Column::CreatedAt.gt(after)
            ])
            .count(conn)
            .await
    }

    pub async fn submission_count_in_my_forms_since<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        user_id: PalformDatabaseID<IDAdminUser>,
        since: DateTime<Utc>,
    ) -> Result<Vec<APISubmissionCountPerForm>, DbErr> {
        Submission::find()
            .join(JoinType::InnerJoin, submission::Relation::Form.def())
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .join(JoinType::InnerJoin, team::Relation::TeamMembership.def())
            .group_by(form::Column::Id)
            .group_by(team::Column::Id)
            .filter(all![
                team_membership::Column::UserId.eq(user_id),
                team::Column::OrganisationId.eq(org_id),
                submission::Column::CreatedAt.gt(since)
            ])
            .select_only()
            .column_as(form::Column::Id, "form_id")
            .column_as(team::Column::Id, "team_id")
            .column_as(submission::Column::Id.count(), "new_submission_count")
            .into_model()
            .all(conn)
            .await
    }

    pub async fn count_for_form<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<u64, DbErr> {
        Submission::find()
            .filter(submission::Column::FormId.eq(form_id))
            .count(conn)
            .await
    }

    pub async fn verify_submission_form<T: ConnectionTrait>(
        conn: &T,
        submission_id: PalformDatabaseID<IDSubmission>,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<bool, DbErr> {
        let resp = Submission::find_by_id(submission_id)
            .filter(submission::Column::FormId.eq(form_id))
            .count(conn)
            .await?;
        Ok(resp == 1)
    }

    pub async fn delete_submission<T: ConnectionTrait>(
        conn: &T,
        id: PalformDatabaseID<IDSubmission>,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<(), DbErr> {
        Submission::delete_by_id(id).exec(conn).await?;

        let new_deletion = deleted_submission::ActiveModel {
            id: Set(id),
            form_id: Set(form_id),
            ..Default::default()
        };
        new_deletion.insert(conn).await?;

        Ok(())
    }

    pub async fn get_by_id<T: ConnectionTrait>(
        conn: &T,
        submission_id: PalformDatabaseID<IDSubmission>,
    ) -> Result<Option<submission::Model>, DbErr> {
        Submission::find_by_id(submission_id).one(conn).await
    }

    pub async fn run_submission_notification<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        form_id: PalformDatabaseID<IDForm>,
        submission_id: PalformDatabaseID<IDSubmission>,
        encrypted_payload: String,
        mail_client: &PalformMailClient,
    ) -> Result<(), SubmissionNotificationError> {
        #[derive(FromQueryResult)]
        struct FormNotificationSettings {
            notification_email: bool,
            notification_webhook_url: Option<String>,
            team_id: PalformDatabaseID<IDTeam>,
            editor_name: String,
        }

        let form_settings: FormNotificationSettings = Form::find_by_id(form_id.clone())
            .select_only()
            .column(form::Column::NotificationEmail)
            .column(form::Column::NotificationWebhookUrl)
            .column(form::Column::TeamId)
            .column(form::Column::EditorName)
            .into_model()
            .one(conn)
            .await?
            .ok_or(DbErr::RecordNotFound("Organisation not found".to_string()))?;

        if let Some(webhook_url) = form_settings.notification_webhook_url {
            let payload = APISubmissionWebhookPayload {
                submission_id,
                form_id: form_id.clone(),
                org_id: org_id.clone(),
                payload: encrypted_payload,
            };
            let m = SubmissionWebhookManager::new(&webhook_url, payload).map_err(|e| {
                SubmissionNotificationError::Webhook(format!(
                    "parse webhook URL: {}",
                    e.to_string()
                ))
            })?;
            send_webhook(m)
                .await
                .map_err(|e| SubmissionNotificationError::Webhook(e.to_string()))?;
        }

        if form_settings.notification_email {
            let email_addresses: Vec<String> = Team::find_by_id(form_settings.team_id)
                .join(JoinType::InnerJoin, team::Relation::TeamMembership.def())
                .join(
                    JoinType::InnerJoin,
                    team_membership::Relation::AdminUser.def(),
                )
                .select_only()
                .column(admin_user::Column::Email)
                .into_tuple()
                .all(conn)
                .await?;

            for email in email_addresses {
                let parsed_to_address: Result<Mailbox, AddressError> = email.parse();
                if let Ok(parsed_to_address) = parsed_to_address {
                    #[derive(Serialize)]
                    struct MessageVariables {
                        form_editor_name: String,
                        org_id: String,
                        form_id: String,
                    }

                    let message_variables = MessageVariables {
                        form_editor_name: form_settings.editor_name.clone(),
                        org_id: org_id.to_string(),
                        form_id: form_id.to_string(),
                    };
                    let message_variables =
                        serde_json::to_string(&message_variables).map_err(|e| {
                            SubmissionNotificationError::Email(format!(
                                "serialize message variables: {}",
                                e.to_string()
                            ))
                        })?;

                    let message = mail_client
                        .get_email_builder()
                        .to(parsed_to_address)
                        .subject("New form response")
                        .header(MailgunHeader::<MailgunTemplateNameHeader>::new(
                            "form_response".to_string(),
                        ))
                        .header(MailgunHeader::<MailgunVariableListHeader>::new(
                            message_variables,
                        ))
                        .header(ContentType::TEXT_HTML)
                        .body(Vec::new())
                        .map_err(|e| {
                            SubmissionNotificationError::Email(format!("build message: {}", e))
                        })?;

                    mail_client.send_email(message).await.map_err(|e| {
                        SubmissionNotificationError::Email(format!("send email: {}", e))
                    })?;
                }
            }
        }

        Ok(())
    }
}
