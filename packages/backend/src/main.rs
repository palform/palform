use std::io::ErrorKind;

use clap::Command;

use crate::{
    api_server::run_api_server,
    billing::client::init_stripe_client,
    config::Config,
    database::init_db,
    jobs::{
        delete_abandoned_emails::job_delete_abandoned_emails,
        delete_old_audit_logs::job_delete_old_audit_logs,
        delete_old_auth_tokens::job_delete_old_auth_tokens,
        delete_old_submissions::job_delete_old_submissions,
        run_org_delete_requests::job_run_org_delete_requests, webhooks::job_run_webhooks,
    },
};

mod actix_util;
mod api;
mod api_entities;
mod api_server;
mod audit;
mod auth;
mod billing;
mod captcha;
mod config;
mod crypto;
mod database;
mod entity_managers;
mod geo;
mod jobs;
mod mail;
mod openapi;
mod palform_s3;
mod routes;

rust_i18n::i18n!("locales", fallback = "en");
mod i18n;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let matches = Command::new("Palform")
        .about("End-to-end encrypted form builder")
        .subcommand_required(true)
        .subcommand(Command::new("server").about("Run the API server"))
        .subcommand(
            Command::new("job")
                .about("Run a job")
                .subcommand_required(true)
                .subcommands(vec![
                    Command::new("delete-abandoned-emails")
                        .about("Delete abandoned/expired email verification requests"),
                    Command::new("delete-old-audit-logs").about("Delete expired audit log entries"),
                    Command::new("delete-old-auth-tokens").about("Delete expired auth tokens"),
                    Command::new("delete-old-submissions")
                        .about("Delete submissions in form with auto-delete enabled"),
                    Command::new("run-org-delete-requests")
                        .about("Execute org delete requests that have finished their grace period"),
                    Command::new("webhooks").about("Run pending webhook jobs"),
                ]),
        )
        .get_matches();

    let config = Config::parse_config();
    let db = init_db(&config).await;
    let stripe = init_stripe_client(&config);

    match matches.subcommand() {
        Some(("job", sub_matches)) => match sub_matches.subcommand() {
            Some(("delete-abandoned-emails", _)) => job_delete_abandoned_emails(&db)
                .await
                .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string())),
            Some(("delete-old-audit-logs", _)) => job_delete_old_audit_logs(&db)
                .await
                .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string())),
            Some(("delete-old-auth-tokens", _)) => job_delete_old_auth_tokens(&db)
                .await
                .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string())),
            Some(("delete-old-submissions", _)) => job_delete_old_submissions(&db)
                .await
                .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string())),
            Some(("run-org-delete-requests", _)) => {
                job_run_org_delete_requests(&db, &stripe, &config)
                    .await
                    .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string()))
            }
            Some(("webhooks", _)) => job_run_webhooks(&db)
                .await
                .map_err(|e| std::io::Error::new(ErrorKind::Other, e.to_string())),
            _ => unreachable!("Subcommands are required"),
        },
        Some(("server", _)) => run_api_server(&config, &db, &stripe).await,
        _ => unreachable!("Invalid command specified"),
    }
}
