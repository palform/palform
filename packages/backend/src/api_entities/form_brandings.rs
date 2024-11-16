use palform_entities::sea_orm_active_enums::{
    FormBrandingBorderIntensityEnum, FormBrandingBorderRoundingEnum, FormBrandingFontSizeEnum,
    FormBrandingSpacingEnum,
};
use palform_tsid::{
    resources::{IDFormBranding, IDTeam, IDTeamAsset},
    tsid::PalformDatabaseID,
};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIFormBranding {
    pub id: PalformDatabaseID<IDFormBranding>,
    pub name: String,
    pub google_font: String,
    pub primary_color: String,
    pub accent_color: Option<String>,
    pub logo_asset_id: Option<PalformDatabaseID<IDTeamAsset>>,
    pub background_image_asset_id: Option<PalformDatabaseID<IDTeamAsset>>,
    pub border_rounding: FormBrandingBorderRoundingEnum,
    pub spacing: FormBrandingSpacingEnum,
    pub font_size: FormBrandingFontSizeEnum,
    pub include_palform_attribution: bool,
    pub terms_link: Option<String>,
    pub privacy_link: Option<String>,
    pub extra_footer_message: Option<String>,
    pub border_intensity: FormBrandingBorderIntensityEnum,
    pub border_shadow_intensity: FormBrandingBorderIntensityEnum,
    pub e2ee_badge: bool,
    pub background_color: Option<String>,
    pub background_color_accent: Option<String>,
}

#[derive(Deserialize, JsonSchema)]
pub struct APIFormBrandingRequest {
    pub name: String,
    pub primary_color: String,
    pub accent_color: Option<String>,
    pub google_font: String,
    pub logo_asset_id: Option<PalformDatabaseID<IDTeamAsset>>,
    pub background_image_asset_id: Option<PalformDatabaseID<IDTeamAsset>>,
    pub border_rounding: FormBrandingBorderRoundingEnum,
    pub spacing: FormBrandingSpacingEnum,
    pub font_size: FormBrandingFontSizeEnum,
    pub include_palform_attribution: bool,
    pub terms_link: Option<String>,
    pub privacy_link: Option<String>,
    pub extra_footer_message: Option<String>,
    pub border_intensity: FormBrandingBorderIntensityEnum,
    pub border_shadow_intensity: FormBrandingBorderIntensityEnum,
    pub e2ee_badge: bool,
    pub background_color: Option<String>,
    pub background_color_accent: Option<String>,
}

#[derive(Serialize, JsonSchema, FromQueryResult)]
pub struct APIFormBrandingAccess {
    pub team_id: PalformDatabaseID<IDTeam>,
    pub team_name: String,
}
