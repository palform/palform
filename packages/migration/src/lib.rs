pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240506_081524_add_keys;
mod m20240507_081709_add_key_fingerprint;
mod m20240510_153821_add_question_polymorph;
mod m20240513_150512_add_question_position;
mod m20240513_163630_add_fill_access_token;
mod m20240524_202623_remove_key_fingerprint;
mod m20240524_213558_add_question_required;
mod m20240525_140833_add_org_invites;
mod m20240526_164448_add_cert_fingerprint;
mod m20240527_091849_add_cert_encrypted_backup;
mod m20240602_090119_add_question_grouping;
mod m20240602_164535_add_question_group_step_strategy;
mod m20240612_143743_add_teams;
mod m20240614_094855_default_team;
mod m20240617_102518_form_branding;
mod m20240617_221135_org_created_at;
mod m20240618_120458_billing;
mod m20240620_180528_audit;
mod m20240621_172712_form_notifications;
mod m20240621_222939_more_branding;
mod m20240622_112236_team_assets;
mod m20240622_121843_branding_asset_id;
mod m20240622_215245_submission_fill_token;
mod m20240623_220858_advanced_auth;
mod m20240624_183921_entitlement_oidc;
mod m20240625_044632_audit_extras;
mod m20240625_102454_short_link;
mod m20240703_091502_entitlement_auto_delete_submission;
mod m20240703_093758_form_auto_delete;
mod m20240704_142910_rename_branding_entitlement;
mod m20240708_124504_add_auth_audit;
mod m20240709_141550_form_end_config;
mod m20240710_122132_2fa;
mod m20240711_085328_branding_attribution;
mod m20240714_173414_new_submission_block;
mod m20240720_211741_custom_api_token;
mod m20240723_085149_form_captcha;
mod m20240728_111828_tsid;
mod m20240730_203522_response_overage;
mod m20240731_142724_branding_sharing;
mod m20240731_161124_branding_accent;
mod m20240731_163640_branding_border_intensity;
mod m20240802_092611_submission_deletion_log;
mod m20240815_132308_branding_e2ee_badge;
mod m20240903_110354_question_internal_name;
mod m20240904_212438_single_question_forms;
mod m20240910_113656_nullable_display_name;
mod m20240918_115812_social_auth;
mod m20240918_214834_form_templates;
mod m20241115_000035_branding_backgrounds;
mod m20241119_233312_webhooks;
mod m20241123_170713_webauthn;
mod m20250308_153858_feedback;
mod m20250928_151739_add_submission_audit;
mod m20250928_171251_add_public_key_audit;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240506_081524_add_keys::Migration),
            Box::new(m20240507_081709_add_key_fingerprint::Migration),
            Box::new(m20240510_153821_add_question_polymorph::Migration),
            Box::new(m20240513_150512_add_question_position::Migration),
            Box::new(m20240513_163630_add_fill_access_token::Migration),
            Box::new(m20240524_202623_remove_key_fingerprint::Migration),
            Box::new(m20240524_213558_add_question_required::Migration),
            Box::new(m20240525_140833_add_org_invites::Migration),
            Box::new(m20240526_164448_add_cert_fingerprint::Migration),
            Box::new(m20240527_091849_add_cert_encrypted_backup::Migration),
            Box::new(m20240602_090119_add_question_grouping::Migration),
            Box::new(m20240602_164535_add_question_group_step_strategy::Migration),
            Box::new(m20240612_143743_add_teams::Migration),
            Box::new(m20240614_094855_default_team::Migration),
            Box::new(m20240617_102518_form_branding::Migration),
            Box::new(m20240617_221135_org_created_at::Migration),
            Box::new(m20240618_120458_billing::Migration),
            Box::new(m20240620_180528_audit::Migration),
            Box::new(m20240621_172712_form_notifications::Migration),
            Box::new(m20240621_222939_more_branding::Migration),
            Box::new(m20240622_112236_team_assets::Migration),
            Box::new(m20240622_121843_branding_asset_id::Migration),
            Box::new(m20240622_215245_submission_fill_token::Migration),
            Box::new(m20240623_220858_advanced_auth::Migration),
            Box::new(m20240624_183921_entitlement_oidc::Migration),
            Box::new(m20240625_044632_audit_extras::Migration),
            Box::new(m20240625_102454_short_link::Migration),
            Box::new(m20240703_091502_entitlement_auto_delete_submission::Migration),
            Box::new(m20240703_093758_form_auto_delete::Migration),
            Box::new(m20240704_142910_rename_branding_entitlement::Migration),
            Box::new(m20240708_124504_add_auth_audit::Migration),
            Box::new(m20240709_141550_form_end_config::Migration),
            Box::new(m20240710_122132_2fa::Migration),
            Box::new(m20240711_085328_branding_attribution::Migration),
            Box::new(m20240714_173414_new_submission_block::Migration),
            Box::new(m20240720_211741_custom_api_token::Migration),
            Box::new(m20240723_085149_form_captcha::Migration),
            Box::new(m20240728_111828_tsid::Migration),
            Box::new(m20240730_203522_response_overage::Migration),
            Box::new(m20240731_142724_branding_sharing::Migration),
            Box::new(m20240731_161124_branding_accent::Migration),
            Box::new(m20240731_163640_branding_border_intensity::Migration),
            Box::new(m20240802_092611_submission_deletion_log::Migration),
            Box::new(m20240815_132308_branding_e2ee_badge::Migration),
            Box::new(m20240903_110354_question_internal_name::Migration),
            Box::new(m20240904_212438_single_question_forms::Migration),
            Box::new(m20240910_113656_nullable_display_name::Migration),
            Box::new(m20240918_115812_social_auth::Migration),
            Box::new(m20240918_214834_form_templates::Migration),
            Box::new(m20241115_000035_branding_backgrounds::Migration),
            Box::new(m20241119_233312_webhooks::Migration),
            Box::new(m20241123_170713_webauthn::Migration),
            Box::new(m20250308_153858_feedback::Migration),
            Box::new(m20250928_151739_add_submission_audit::Migration),
            Box::new(m20250928_171251_add_public_key_audit::Migration),
        ]
    }
}
