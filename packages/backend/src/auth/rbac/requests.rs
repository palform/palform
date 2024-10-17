use std::{fmt::Display, marker::PhantomData, ops::Deref};

use palform_client_common::errors::error::APIError;
use palform_entities::sea_orm_active_enums::OrganisationMemberRoleEnum;
use palform_tsid::{
    resources::{IDForm, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use rocket::{
    outcome::try_outcome,
    request::{self, FromRequest},
    serde::json::Json,
};
use rocket_okapi::{
    okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData},
    request::{OpenApiFromRequest, RequestHeaderInput},
};
use sea_orm::DatabaseConnection;

use crate::{
    auth::{
        rbac::teams_manager::TeamsRBACManager,
        tokens::{APIAuthToken, APIAuthTokenSource, APIAuthTokenSourceAny},
    },
    entity_managers::forms::FormManager,
    into_outcome,
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

#[rocket::async_trait]
impl<'a, Role: TeamRoleType, Target: OrgRoleTargetType, Source: APIAuthTokenSource> FromRequest<'a>
    for OrgRoleToken<Role, Target, Source>
{
    type Error = Json<APIError>;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        let api_token = try_outcome!(APIAuthToken::<Source>::from_request(request).await);

        let db = into_outcome!(
            request
                .rocket()
                .state::<DatabaseConnection>()
                .ok_or_else(|| APIError::report_internal_error_without_error(
                    "Database not found in state oopsies"
                )),
            request
        );

        let org_id = into_outcome!(
            request
                .param::<PalformDatabaseID<IDOrganisation>>(3)
                .ok_or(APIError::BadRequest(
                    "Not enough parameters for org ID".to_string(),
                ))
                .and_then(|e| e.map_err(|e| APIError::BadRequest(e.to_string()))),
            request
        );

        let target = Target::target();
        if target == OrgRoleTarget::OrgViewAndTeamRoleFromForm
            || target == OrgRoleTarget::OrgViewAndTeamRoleFromTeam
        {
            let team_id: PalformDatabaseID<IDTeam> =
                if target == OrgRoleTarget::OrgViewAndTeamRoleFromForm {
                    let form_id = into_outcome!(
                        request
                            .param::<PalformDatabaseID<IDForm>>(5)
                            .ok_or(APIError::BadRequest(
                                "Not enough parameters for form ID".to_string()
                            ))
                            .and_then(|e| e.map_err(|e| APIError::BadRequest(e.to_string()))),
                        request
                    );

                    into_outcome!(
                        FormManager::get_form_team_id(db, form_id)
                            .await
                            .map_err(|e| APIError::BadRequest(e.to_string())),
                        request
                    )
                } else {
                    into_outcome!(
                        request
                            .param::<PalformDatabaseID<IDTeam>>(5)
                            .ok_or(APIError::BadRequest(
                                "Not enough parameters for team ID".to_string()
                            ))
                            .and_then(|e| e.map_err(|e| APIError::BadRequest(e.to_string()))),
                        request
                    )
                };

            let target_role = Role::role().expect("missing team role in team role RBAC token");
            into_outcome!(
                TeamsRBACManager::from(api_token.clone())
                    .require_in_request(db, team_id, org_id, target_role)
                    .await,
                request
            );
        } else if target == OrgRoleTarget::OrgView || target == OrgRoleTarget::OrgAdmin {
            let m = OrgsRBACManager {
                org_id,
                user_id: api_token.source.get_user_id(),
            };

            into_outcome!(
                m.require_in_request(db, target == OrgRoleTarget::OrgAdmin)
                    .await,
                request
            );
        }

        request::Outcome::Success(OrgRoleToken::<Role, Target, Source> {
            token: api_token,
            role: PhantomData,
            target: PhantomData,
        })
    }
}

impl<'a, Role: TeamRoleType, Target: OrgRoleTargetType, Source: APIAuthTokenSource>
    OpenApiFromRequest<'a> for OrgRoleToken<Role, Target, Source>
{
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        let mut description = "Requires authentication to access.".to_string();

        if Source::allow_personal() {
            description += " Allows frontend user tokens (HTTP basic auth) generated with the authentication endpoints.";
        }
        if Source::allow_service_account() {
            description += " Allows service-account tokens via the public Palform API (HTTP bearer auth). Please use the format `Bearer {service_account_token}`.";
        }

        if let Some(role) = Role::role() {
            match role {
                OrganisationMemberRoleEnum::Viewer => description += " Requires viewing permission",
                OrganisationMemberRoleEnum::Editor => description += " Requires editing permission",
                OrganisationMemberRoleEnum::Admin => description += " Requires admin permission",
            };

            description += " on the team";
        }

        match Target::target() {
            OrgRoleTarget::OrgView => {
                description += "Requires viewing permission on the organisation."
            }
            OrgRoleTarget::OrgAdmin => {
                description += "Requires admin permission on the organisation."
            }
            OrgRoleTarget::OrgViewAndTeamRoleFromForm => description += " owning the form.",
            OrgRoleTarget::OrgViewAndTeamRoleFromTeam => description += " in the request.",
        }

        let security_scheme = SecurityScheme {
            description: Some(description),
            data: SecuritySchemeData::Http {
                scheme: if Source::allow_personal() {
                    "basic".to_owned()
                } else {
                    "bearer".to_owned()
                },
                bearer_format: None,
            },
            extensions: Object::default(),
        };

        let mut security_req = SecurityRequirement::new();

        let security_name = format!(
            "APIAuthTokenWithRole_{}_{}_{}",
            if let Some(role) = Role::role() {
                format!("{:?}", role)
            } else {
                "None".to_string()
            },
            Target::target(),
            Source::to_string()
        );
        security_req.insert(security_name.to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            security_name,
            security_scheme,
            security_req,
        ))
    }
}
