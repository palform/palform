use palform_entities::{form_branding, form_branding_team_access, prelude::*, team};
use palform_tsid::{
    resources::{IDFormBranding, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbErr, EntityTrait, JoinType, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait, Set,
};

use crate::api_entities::form_brandings::{
    APIFormBranding, APIFormBrandingAccess, APIFormBrandingRequest,
};

use super::billing_entitlement_proxy::BillingEntitlementCountTrait;

impl BillingEntitlementCountTrait for FormBrandingManager {
    async fn billing_count<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<u64, DbErr> {
        FormBranding::find()
            .join(
                JoinType::InnerJoin,
                form_branding::Relation::FormBrandingTeamAccess.def(),
            )
            .join(
                JoinType::InnerJoin,
                form_branding_team_access::Relation::Team.def(),
            )
            .filter(team::Column::OrganisationId.eq(org_id))
            .count(conn)
            .await
    }
}

pub struct FormBrandingManager;

impl FormBrandingManager {
    pub async fn list_in_team<T: ConnectionTrait>(
        conn: &T,
        team_id: PalformDatabaseID<IDTeam>,
    ) -> Result<Vec<APIFormBranding>, DbErr> {
        FormBranding::find()
            .join(
                JoinType::InnerJoin,
                form_branding::Relation::FormBrandingTeamAccess.def(),
            )
            .filter(form_branding_team_access::Column::TeamId.eq(team_id))
            .into_model()
            .all(conn)
            .await
    }

    pub async fn get_by_id<T: ConnectionTrait>(
        conn: &T,
        branding_id: PalformDatabaseID<IDFormBranding>,
    ) -> Result<Option<APIFormBranding>, DbErr> {
        FormBranding::find_by_id(branding_id)
            .into_model()
            .one(conn)
            .await
    }

    pub async fn verify_branding_team_allowed<T: ConnectionTrait>(
        conn: &T,
        branding_id: PalformDatabaseID<IDFormBranding>,
        team_id: PalformDatabaseID<IDTeam>,
    ) -> Result<bool, DbErr> {
        FormBranding::find_by_id(branding_id)
            .join(
                JoinType::InnerJoin,
                form_branding::Relation::FormBrandingTeamAccess.def(),
            )
            .filter(form_branding_team_access::Column::TeamId.eq(team_id))
            .count(conn)
            .await
            .map(|c| c == 1)
    }

    fn active_model_from_request(
        existing_id: Option<PalformDatabaseID<IDFormBranding>>,
        data: APIFormBrandingRequest,
    ) -> form_branding::ActiveModel {
        form_branding::ActiveModel {
            id: Set(existing_id.unwrap_or(PalformDatabaseID::<IDFormBranding>::random())),
            name: Set(data.name),
            google_font: Set(data.google_font),
            primary_color: Set(data.primary_color),
            accent_color: Set(data.accent_color),
            logo_asset_id: Set(data.logo_asset_id),
            background_image_asset_id: Set(data.background_image_asset_id),
            border_rounding: Set(data.border_rounding),
            spacing: Set(data.spacing),
            font_size: Set(data.font_size),
            include_palform_attribution: Set(data.include_palform_attribution),
            terms_link: Set(data.terms_link),
            privacy_link: Set(data.privacy_link),
            extra_footer_message: Set(data.extra_footer_message),
            border_intensity: Set(data.border_intensity),
            border_shadow_intensity: Set(data.border_shadow_intensity),
            e2ee_badge: Set(data.e2ee_badge),
            background_color: Set(data.background_color),
            background_color_accent: Set(data.background_color_accent),
        }
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        data: APIFormBrandingRequest,
    ) -> Result<PalformDatabaseID<IDFormBranding>, DbErr> {
        let new_model = Self::active_model_from_request(None, data);
        let result = new_model.insert(conn).await?;
        Ok(result.id)
    }

    pub async fn update<T: ConnectionTrait>(
        conn: &T,
        branding_id: PalformDatabaseID<IDFormBranding>,
        data: APIFormBrandingRequest,
    ) -> Result<(), DbErr> {
        let updated_model = Self::active_model_from_request(Some(branding_id), data);
        updated_model.update(conn).await?;
        Ok(())
    }

    pub async fn delete<T: ConnectionTrait>(
        conn: &T,
        branding_id: PalformDatabaseID<IDFormBranding>,
    ) -> Result<(), DbErr> {
        FormBranding::delete_by_id(branding_id)
            .exec(conn)
            .await
            .map(|_| ())
    }

    pub async fn add_access<T: ConnectionTrait>(
        conn: &T,
        branding_id: PalformDatabaseID<IDFormBranding>,
        team_id: PalformDatabaseID<IDTeam>,
    ) -> Result<APIFormBrandingAccess, DbErr> {
        let new_access = form_branding_team_access::ActiveModel {
            form_branding_id: Set(branding_id),
            team_id: Set(team_id),
        };
        new_access.insert(conn).await?;

        let new_access = FormBrandingTeamAccess::find_by_id((branding_id, team_id))
            .join(
                JoinType::InnerJoin,
                form_branding_team_access::Relation::Team.def(),
            )
            .select_only()
            .column_as(team::Column::Id, "team_id")
            .column_as(team::Column::Name, "team_name")
            .into_model()
            .one(conn)
            .await?
            .ok_or(DbErr::RecordNotFound(
                "Form Branding access we just created".to_string(),
            ))?;
        Ok(new_access)
    }

    pub async fn remove_access<T: ConnectionTrait>(
        conn: &T,
        branding_id: PalformDatabaseID<IDFormBranding>,
        team_id: PalformDatabaseID<IDTeam>,
    ) -> Result<(), DbErr> {
        FormBrandingTeamAccess::delete_by_id((branding_id, team_id))
            .exec(conn)
            .await?;
        Ok(())
    }

    pub async fn list_accessing_teams<T: ConnectionTrait>(
        conn: &T,
        branding_id: PalformDatabaseID<IDFormBranding>,
    ) -> Result<Vec<APIFormBrandingAccess>, DbErr> {
        FormBrandingTeamAccess::find()
            .join(
                JoinType::InnerJoin,
                form_branding_team_access::Relation::Team.def(),
            )
            .filter(form_branding_team_access::Column::FormBrandingId.eq(branding_id))
            .select_only()
            .column_as(form_branding_team_access::Column::TeamId, "team_id")
            .column_as(team::Column::Name, "team_name")
            .into_model()
            .all(conn)
            .await
    }
}
