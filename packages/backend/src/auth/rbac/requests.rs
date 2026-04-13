use std::{fmt::Display, future::Future, marker::PhantomData, ops::Deref, pin::Pin};

use actix_web::{dev::Payload, web::Data, FromRequest, HttpRequest};
use apistos::ApiSecurity;
use palform_client_common::errors::error::APIError;
use palform_entities::sea_orm_active_enums::OrganisationMemberRoleEnum;
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use sea_orm::DatabaseConnection;

use crate::{
    auth::{
        rbac::teams_manager::TeamsRBACManager,
        tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourceAny},
    },
    entity_managers::forms::FormManager,
};

use super::orgs_manager::OrgsRBACManager;

#[derive(PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum OrgRoleTarget {
    /// Derive the Team ID from the form in the request path and validate the role
    OrgViewAndTeamRoleFromForm,
    /// Derive the Team ID from the team ID in the request path and validate the role
    OrgViewAndTeamRoleFromTeam,
    /// Derive the Org ID from the request path and validate org membership existence
    OrgView,
    /// Derive the Org ID from the request path and validate is_admin=true org membership
    OrgAdmin,
}
impl Display for OrgRoleTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OrgRoleTarget::OrgViewAndTeamRoleFromForm => write!(f, "OrgViewAndTeamRoleFromForm"),
            OrgRoleTarget::OrgViewAndTeamRoleFromTeam => write!(f, "OrgViewAndTeamRoleFromTeam"),
            OrgRoleTarget::OrgView => write!(f, "OrgView"),
            OrgRoleTarget::OrgAdmin => write!(f, "OrgAdmin"),
        }
    }
}

pub trait OrgRoleTargetType {
    fn target() -> OrgRoleTarget;
}

pub struct TokenTargetTeamFromForm;
impl OrgRoleTargetType for TokenTargetTeamFromForm {
    fn target() -> OrgRoleTarget {
        OrgRoleTarget::OrgViewAndTeamRoleFromForm
    }
}

pub struct TokenTargetTeamFromTeam;
impl OrgRoleTargetType for TokenTargetTeamFromTeam {
    fn target() -> OrgRoleTarget {
        OrgRoleTarget::OrgViewAndTeamRoleFromTeam
    }
}

pub struct TokenTargetOrgView;
impl OrgRoleTargetType for TokenTargetOrgView {
    fn target() -> OrgRoleTarget {
        OrgRoleTarget::OrgView
    }
}

pub struct TokenTargetOrgAdmin;
impl OrgRoleTargetType for TokenTargetOrgAdmin {
    fn target() -> OrgRoleTarget {
        OrgRoleTarget::OrgAdmin
    }
}

pub trait TeamRoleType {
    fn role() -> Option<OrganisationMemberRoleEnum>;
}

pub struct TeamRoleViewer;
impl TeamRoleType for TeamRoleViewer {
    fn role() -> Option<OrganisationMemberRoleEnum> {
        Some(OrganisationMemberRoleEnum::Viewer)
    }
}
pub struct TeamRoleEditor;
impl TeamRoleType for TeamRoleEditor {
    fn role() -> Option<OrganisationMemberRoleEnum> {
        Some(OrganisationMemberRoleEnum::Editor)
    }
}
pub struct TeamRoleAdmin;
impl TeamRoleType for TeamRoleAdmin {
    fn role() -> Option<OrganisationMemberRoleEnum> {
        Some(OrganisationMemberRoleEnum::Admin)
    }
}
pub struct TeamRoleNone;
impl TeamRoleType for TeamRoleNone {
    fn role() -> Option<OrganisationMemberRoleEnum> {
        None
    }
}

pub type APITokenTeamViewerFromForm =
    OrgRoleToken<TeamRoleViewer, TokenTargetTeamFromForm, APIAuthTokenSourceAny>;
pub type APITokenTeamViewerFromTeam =
    OrgRoleToken<TeamRoleViewer, TokenTargetTeamFromTeam, APIAuthTokenSourceAny>;
pub type APITokenTeamEditorFromForm =
    OrgRoleToken<TeamRoleEditor, TokenTargetTeamFromForm, APIAuthTokenSourceAny>;
pub type APITokenTeamEditorFromTeam =
    OrgRoleToken<TeamRoleEditor, TokenTargetTeamFromTeam, APIAuthTokenSourceAny>;
pub type APITokenTeamAdminFromTeam =
    OrgRoleToken<TeamRoleAdmin, TokenTargetTeamFromTeam, APIAuthTokenSourceAny>;
pub type APITokenOrgViewer = OrgRoleToken<TeamRoleNone, TokenTargetOrgView, APIAuthTokenSourceAny>;
pub type APITokenOrgAdmin = OrgRoleToken<TeamRoleNone, TokenTargetOrgAdmin, APIAuthTokenSourceAny>;

#[derive(ApiSecurity)]
#[openapi_security(scheme(security_type(http(scheme = "basic"))))]
pub struct OrgRoleToken<Role: TeamRoleType, Target: OrgRoleTargetType, Source: APIAuthTokenSource> {
    pub token: APIAuthToken<Source>,
    role: PhantomData<Role>,
    target: PhantomData<Target>,
}

impl<Role: TeamRoleType, Target: OrgRoleTargetType, Source: APIAuthTokenSource> Deref
    for OrgRoleToken<Role, Target, Source>
{
    type Target = APIAuthToken<Source>;
    fn deref(&self) -> &Self::Target {
        &self.token
    }
}

impl<Role: TeamRoleType, Target: OrgRoleTargetType, Source: APIAuthTokenSource + 'static>
    FromRequest for OrgRoleToken<Role, Target, Source>
{
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;
    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let token_fut = APIAuthToken::<Source>::from_request(req, payload);
        let req = req.clone();

        Box::pin(async move {
            let api_token = token_fut.await?;

            let db = req
                .app_data::<Data<DatabaseConnection>>()
                .ok_or_else(|| APIError::report_internal_error_without_error("DB not in state"))?;

            let org_id: PalformDatabaseID<IDOrganisation> = req
                .match_info()
                .get("org_id")
                .ok_or(APIError::BadRequest("Org ID not in path".to_string()))?
                .parse()
                .map_err(|e| APIError::BadRequest(format!("Invalid org ID: {}", e)))?;

            let target = Target::target();
            if target == OrgRoleTarget::OrgViewAndTeamRoleFromForm
                || target == OrgRoleTarget::OrgViewAndTeamRoleFromTeam
            {
                let team_id: PalformDatabaseID<IDTeam> =
                    if target == OrgRoleTarget::OrgViewAndTeamRoleFromForm {
                        let form_id: PalformDatabaseID<IDForm> = req
                            .match_info()
                            .get("form_id")
                            .ok_or(APIError::BadRequest("Form ID not in path".to_string()))?
                            .parse()
                            .map_err(|e| APIError::BadRequest(format!("Invalid form ID: {}", e)))?;

                        FormManager::get_form_team_id(db.as_ref(), form_id)
                            .await
                            .map_err(|e| APIError::BadRequest(e.to_string()))?
                    } else {
                        req.match_info()
                            .get("team_id")
                            .ok_or(APIError::BadRequest("Team ID not in path".to_string()))?
                            .parse()
                            .map_err(|e| APIError::BadRequest(format!("Invalid team ID: {}", e)))?
                    };

                let target_role = Role::role().expect("missing team role in team role RBAC token");
                TeamsRBACManager::from(api_token.clone())
                    .require_in_request(db.as_ref(), team_id, org_id, target_role)
                    .await?;
            } else if target == OrgRoleTarget::OrgView || target == OrgRoleTarget::OrgAdmin {
                let m = OrgsRBACManager {
                    org_id,
                    user_id: api_token.source.get_user_id(),
                };

                m.require_in_request(db.as_ref(), target == OrgRoleTarget::OrgAdmin)
                    .await?;
            }

            Ok(OrgRoleToken::<Role, Target, Source> {
                token: api_token,
                role: PhantomData,
                target: PhantomData,
            })
        })
    }
}
