use clap::Command;
use config::Config;
use database::init_db;
use jobs::{
    delete_abandoned_emails::job_delete_abandoned_emails,
    delete_old_audit_logs::job_delete_old_audit_logs,
    delete_old_auth_tokens::job_delete_old_auth_tokens,
    delete_old_submissions::job_delete_old_submissions,
};
use mail::client::PalformMailClient;
use palform_s3::{
    buckets::{S3BucketSubmissionAssets, S3BucketTeamAssets},
    client::PalformS3Client,
};
use rocket::{catchers, get, http::Method, routes};
use rocket_cors::{AllowedOrigins, CorsOptions};
use rocket_okapi::{
    get_openapi_route,
    okapi::{merge::marge_spec_list, openapi3::OpenApi},
    openapi, openapi_get_routes_spec,
    rapidoc::{
        make_rapidoc, GeneralConfig, HideShowConfig, Layout, LayoutConfig, RapiDocConfig,
        RenderStyle,
    },
    settings::{OpenApiSettings, UrlObject},
};
use rocket_util::catchers::default_catcher;

mod api;
mod api_entities;
mod audit;
mod auth;
mod captcha;
mod config;
mod crypto;
mod database;
mod entity_managers;
mod jobs;
mod mail;
mod palform_s3;
mod rocket_util;
mod webhooks;

#[cfg(feature = "saas")]
mod billing;

rust_i18n::i18n!("locales", fallback = "en");
mod i18n;

#[openapi(tag = "Meta", operation_id = "meta.health")]
#[get("/healthz")]
fn health() -> &'static str {
    "all is healthy"
}

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
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
                ]),
        )
        .get_matches();

    let config = Config::parse_config();
    let db = init_db(&config).await;

    match matches.subcommand() {
        Some(("job", sub_matches)) => match sub_matches.subcommand() {
            Some(("delete-abandoned-emails", _)) => job_delete_abandoned_emails(&db).await,
            Some(("delete-old-audit-logs", _)) => job_delete_old_audit_logs(&db).await,
            Some(("delete-old-auth-tokens", _)) => job_delete_old_auth_tokens(&db).await,
            Some(("delete-old-submissions", _)) => job_delete_old_submissions(&db).await,
            _ => unreachable!("Subcommands are required"),
        }
        .unwrap(),
        Some(("server", _)) => {
            let allowed_origins = AllowedOrigins::some_regex(&[config.cors_origin.clone()]);
            let cors = CorsOptions {
                allowed_origins,
                allowed_methods: vec![
                    Method::Get,
                    Method::Post,
                    Method::Put,
                    Method::Patch,
                    Method::Delete,
                    Method::Options,
                ]
                .into_iter()
                .map(From::from)
                .collect(),
                allow_credentials: true,
                max_age: Some(600),
                ..Default::default()
            }
            .to_cors()
            .expect("Configure CORS");

            #[cfg(feature = "saas")]
            let stripe_client = billing::client::init_stripe_client(&config);

            let mail_client = PalformMailClient::new(config.clone()).await;
            let s3_brand_assets =
                PalformS3Client::<S3BucketTeamAssets>::init(&config).expect("Init S3 brand assets");
            let s3_submission_assets = PalformS3Client::<S3BucketSubmissionAssets>::init(&config)
                .expect("Init S3 submission assets");

            let mut r = rocket::build()
                .manage(config)
                .manage(db)
                .manage(mail_client)
                .manage(s3_brand_assets)
                .manage(s3_submission_assets)
                // Some routes are not yet supported by okapi (e.g. due to multipart files)
                .mount(
                    "/",
                    routes![
                        api::team_assets::upload::handler,
                        api::submissions::assets::upload::handler,
                    ],
                );

            let mut route_lists = Vec::<(Vec<rocket::Route>, OpenApi)>::new();
            let main_routes = openapi_get_routes_spec![
                health,
                api::admin_users::update::handler,
                api::auth::start_auth::handler,
                api::auth::auth_callback::handler,
                api::auth::create_user::handler,
                api::auth::resend_verification::handler,
                api::auth::verify_email::handler,
                api::auth::test::handler,
                api::auth::sign_in::handler,
                api::auth::invalidate::handler,
                api::auth::second_factors::list::handler,
                api::auth::second_factors::enroll::handler,
                api::auth::second_factors::delete::handler,
                api::auth::password_reset::send::handler,
                api::auth::password_reset::reset::handler,
                api::auth::social::list::handler,
                api::auth::social::start::handler,
                api::auth::social::callback::handler,
                api::keys::list::handler,
                api::keys::list_org::handler,
                api::keys::register::handler,
                api::keys::register_backup::handler,
                api::keys::delete::handler,
                api::keys::get::handler,
                api::keys::get_backup::handler,
                api::organisations::create::handler,
                api::organisations::list::handler,
                api::organisations::get::handler,
                api::organisations::resolve_subdomain::handler,
                api::organisations::create_subdomain::handler,
                api::organisations::patch::handler,
                api::organisations::delete::handler,
                api::organisation_auth_config::get::handler,
                api::organisation_auth_config::put::handler,
                api::organisation_auth_team_mappings::list::handler,
                api::organisation_auth_team_mappings::create::handler,
                api::organisation_auth_team_mappings::delete::handler,
                api::organisation_members::list::handler,
                api::organisation_members::delete::handler,
                api::organisation_members::join::handler,
                api::organisation_members::am_i_admin::handler,
                api::organisation_members::patch::handler,
                api::organisation_invites::create::handler,
                api::organisation_invites::delete::handler,
                api::organisation_invites::list::handler,
                api::organisation_invites::preview::handler,
                api::organisation_team_members::add::handler,
                api::organisation_team_members::list::handler,
                api::organisation_team_members::patch::handler,
                api::organisation_team_members::delete::handler,
                api::organisation_teams::list_my::handler,
                api::organisation_teams::list::handler,
                api::organisation_teams::get::handler,
                api::organisation_teams::create::handler,
                api::organisation_teams::delete::handler,
                api::team_assets::list::handler,
                api::team_assets::get_fill::handler,
                api::team_assets::get::handler,
                api::form_brandings::list::handler,
                api::form_brandings::list_fonts::handler,
                api::form_brandings::create::handler,
                api::form_brandings::delete::handler,
                api::form_brandings::put::handler,
                api::form_brandings::add_access::handler,
                api::form_brandings::list_access::handler,
                api::form_brandings::remove_access::handler,
                api::forms::list::handler,
                api::forms::get::handler,
                api::forms::view::handler,
                api::forms::keys::handler,
                api::forms::fill::handler,
                api::forms::create::handler,
                api::forms::update::handler,
                api::forms::delete::handler,
                api::forms::relocate::handler,
                api::forms::exchange_short_link::handler,
                api::forms::set_auto_delete::handler,
                api::form_templates::list_categories::handler,
                api::form_templates::get_category::handler,
                api::form_templates::list::handler,
                api::form_templates::list_top::handler,
                api::form_templates::get::handler,
                api::form_templates::report_view::handler,
                api::form_templates::clone::handler,
                api::submissions::crypto::handler,
                api::submissions::list::handler,
                api::submissions::delete::handler,
                api::submissions::num_since::handler,
                api::submissions::assets::download::handler,
                api::questions::get::handler,
                api::questions::list::handler,
                api::questions::save::handler,
                api::question_groups::list::handler,
                api::fill_tokens::list::handler,
                api::fill_tokens::create::handler,
                api::fill_tokens::delete::handler,
                api::induction::status::handler,
                api::induction::alert::handler,
                api::audit::list::handler,
            ];
            route_lists.push(main_routes);

            #[cfg(feature = "saas")]
            {
                r = r.manage(stripe_client);
                let extra_routes = openapi_get_routes_spec![
                    api::billing::plans::list::handler,
                    api::billing::plans::get::handler,
                    api::billing::plans::initiate::handler,
                    api::billing::plans::cancel::handler,
                    api::billing::plans::switch::handler,
                    api::billing::customers::get::handler,
                    api::billing::customers::update_payment_method::handler,
                    api::billing::invoices::list::handler,
                    api::billing::invoices::preview::handler,
                    api::billing::entitlements::test::handler,
                    api::billing::entitlements::list::handler,
                ];
                route_lists.push(extra_routes);
                r = r.mount("/", routes![api::billing::webhooks::receiver::handler]);
            }

            #[cfg(feature = "country-metadata")]
            {
                let extra_routes = openapi_get_routes_spec![
                    api::countries::list_calling_codes::handler,
                    api::countries::list_names::handler
                ];
                route_lists.push(extra_routes);
            }

            let mut doc_list = Vec::<(String, OpenApi)>::new();
            for (routes, doc) in route_lists {
                r = r.mount("/", routes);
                doc_list.push(("/".to_string(), doc));
            }
            let openapi_doc = marge_spec_list(&doc_list).expect("Merge OpenAPI doc list");

            r = r.mount(
                "/",
                vec![get_openapi_route(openapi_doc, &OpenApiSettings::default())],
            );

            r.mount(
                "/rapidoc/",
                make_rapidoc(&RapiDocConfig {
                    general: GeneralConfig {
                        spec_urls: vec![
                            UrlObject::new("General", "../openapi.json"),
                            UrlObject::new("SaaS", "../saas/openapi.json"),
                        ],
                        persist_auth: true,
                        ..Default::default()
                    },
                    hide_show: HideShowConfig {
                        allow_spec_url_load: false,
                        ..Default::default()
                    },
                    layout: LayoutConfig {
                        layout: Layout::Row,
                        render_style: RenderStyle::View,
                        ..Default::default()
                    },
                    ..Default::default()
                }),
            )
            .attach(cors)
            .register("/", catchers![default_catcher])
            .launch()
            .await?;
        }
        _ => unreachable!("Subcommands are required"),
    }

    Ok(())
}
