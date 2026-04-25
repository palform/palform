use apistos::web::{scope, Scope};

use crate::api::{
    admin_users::scope::admin_users_scope,
    audit::scope::audit_logs_scope,
    auth::scope::auth_scope,
    billing::scope::{billing_public_scope, billing_scope},
    countries::scope::countries_scope,
    feedback::scope::feedback_scope,
    fill_tokens::scope::fill_tokens_scope,
    form_brandings::scope::{form_brandings_google_fonts_scope, form_brandings_scope},
    form_templates::scope::{form_templates_clone_scope, form_templates_public_scope},
    forms::scope::{forms_fill_scope, forms_org_scope},
    health::scope::health_scope,
    induction::scope::induction_scope,
    keys::scope::{organisation_keys_scope, organisation_team_keys_scope},
    organisation_auth_config::scope::organisation_auth_config_scope,
    organisation_auth_team_mappings::scope::organisation_auth_team_mappings_scope,
    organisation_invites::scope::organisation_invites_scope,
    organisation_members::scope::organisation_members_scope,
    organisation_team_members::scope::organisation_team_members_scope,
    organisation_teams::scope::organisation_teams_scope,
    organisations::scope::organisations_scope,
    question_groups::scope::question_groups_scope,
    questions::scope::questions_scope,
    submissions::scope::{
        submissions_fill_assets_scope, submissions_form_scope, submissions_org_scope,
    },
    team_assets::scope::{
        team_assets_fill_legacy_scope, team_assets_fill_scope, team_assets_scope,
    },
    webhooks::scope::webhooks_scope,
};

pub fn main_routes() -> Scope {
    scope("api")
        .service(health_scope())
        .service(auth_scope())
        .service(admin_users_scope())
        .service(feedback_scope())
        .service(form_brandings_google_fonts_scope())
        .service(countries_scope())
        .service(form_templates_public_scope())
        .service(billing_public_scope())
        .service(
            scope("org").service(organisations_scope()).service(
                scope("{org_id}")
                    .service(organisation_auth_config_scope())
                    .service(organisation_auth_team_mappings_scope())
                    .service(organisation_invites_scope())
                    .service(organisation_members_scope())
                    .service(induction_scope())
                    .service(audit_logs_scope())
                    .service(organisation_keys_scope())
                    .service(submissions_org_scope())
                    .service(billing_scope())
                    .service(form_templates_clone_scope())
                    .service(
                        scope("team").service(organisation_teams_scope()).service(
                            scope("{team_id}")
                                .service(team_assets_scope())
                                .service(organisation_team_keys_scope())
                                .service(organisation_team_members_scope())
                                .service(form_brandings_scope()),
                        ),
                    )
                    .service(
                        scope("form").service(forms_org_scope()).service(
                            scope("{form_id}")
                                .service(submissions_form_scope())
                                .service(questions_scope())
                                .service(question_groups_scope())
                                .service(webhooks_scope())
                                .service(fill_tokens_scope()),
                        ),
                    ),
            ),
        )
        .service(
            scope("fill").service(
                scope("form").service(forms_fill_scope()).service(
                    scope("{form_id}")
                        .service(team_assets_fill_scope())
                        .service(scope("org/{org_id}").service(submissions_fill_assets_scope())),
                ),
            ),
        )
}

pub fn legacy_compat_routes() -> Scope {
    scope("fill/orgs/{org_id}/forms/{form_id}").service(team_assets_fill_legacy_scope())
}
