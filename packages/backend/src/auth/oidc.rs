use std::collections::{HashMap, HashSet};

use openidconnect::{
    core::{
        CoreAuthDisplay, CoreAuthPrompt, CoreAuthenticationFlow, CoreErrorResponseType,
        CoreGenderClaim, CoreJsonWebKey, CoreJsonWebKeyType, CoreJsonWebKeyUse,
        CoreJweContentEncryptionAlgorithm, CoreJwsSigningAlgorithm, CoreProviderMetadata,
        CoreRevocableToken, CoreRevocationErrorResponse, CoreTokenIntrospectionResponse,
        CoreTokenType,
    },
    reqwest::async_http_client,
    AdditionalClaims, Client, ClientId, ClientSecret, CsrfToken, EmptyExtraTokenFields,
    IdTokenClaims, IdTokenFields, IssuerUrl, Nonce, RedirectUrl, Scope, StandardErrorResponse,
    StandardTokenResponse,
};
use palform_entities::sea_orm_active_enums::OrganisationMemberRoleEnum;
use palform_tsid::{
    resources::{IDAdminUser, IDOrganisation, IDTeam},
    tsid::PalformDatabaseID,
};
use sea_orm::{ConnectionTrait, DbErr};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

use crate::{
    api_entities::{
        organisation_auth_config::APIOrganisationAuthConfig,
        organisation_auth_team_mapping::APIOrganisationAuthTeamMapping,
        organisation_team::APIOrganisationTeamMembership,
    },
    auth::tokens::{NewAPIAuthToken, TokenManager},
    entity_managers::{
        admin_users::{AdminUserManagementError, AdminUserManager},
        organisation_auth_config::OrganisationAuthConfigManager,
        organisation_auth_team_mappings::OrganisationAuthTeamMappingsManager,
        organisation_members::OrganisationMembersManager,
        organisation_teams::OrganisationTeamsManager,
    },
    rocket_util::from_org_id::FromOrgIdTrait,
};

use super::oidc_common::{oidc_common_token_exchange, TokenExchangeError};

#[derive(Debug, Error)]
pub enum GetClientError {
    #[error("{0}")]
    DBError(#[from] DbErr),
    #[error("Organisation not found or did not have auth configuration")]
    OrgNotFound,
    #[error("Parse URL: {0}")]
    ParseURL(#[from] openidconnect::url::ParseError),
    #[error("Discovery: {0}")]
    DiscoveryError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct GroupsClaim(HashMap<String, serde_json::Value>);
impl AdditionalClaims for GroupsClaim {}

type GroupsClaimClient = Client<
    GroupsClaim,
    CoreAuthDisplay,
    CoreGenderClaim,
    CoreJweContentEncryptionAlgorithm,
    CoreJwsSigningAlgorithm,
    CoreJsonWebKeyType,
    CoreJsonWebKeyUse,
    CoreJsonWebKey,
    CoreAuthPrompt,
    StandardErrorResponse<CoreErrorResponseType>,
    StandardTokenResponse<
        IdTokenFields<
            GroupsClaim,
            EmptyExtraTokenFields,
            CoreGenderClaim,
            CoreJweContentEncryptionAlgorithm,
            CoreJwsSigningAlgorithm,
            CoreJsonWebKeyType,
        >,
        CoreTokenType,
    >,
    CoreTokenType,
    CoreTokenIntrospectionResponse,
    CoreRevocableToken,
    CoreRevocationErrorResponse,
>;

pub struct OIDCManager {
    org_id: PalformDatabaseID<IDOrganisation>,
    client: GroupsClaimClient,
    config: APIOrganisationAuthConfig,
}

impl OIDCManager {
    pub async fn get_client_for_org<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
    ) -> Result<Self, GetClientError> {
        let m = OrganisationAuthConfigManager::new(org_id);
        let org_auth = m.get(conn).await?.ok_or(GetClientError::OrgNotFound)?;

        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(org_auth.oidc_discovery_url.clone())?,
            async_http_client,
        )
        .await
        .map_err(|e| GetClientError::DiscoveryError(e.to_string()))?;

        let client: GroupsClaimClient = Client::from_provider_metadata(
            provider_metadata,
            ClientId::new(org_auth.client_id.clone()),
            Some(ClientSecret::new(org_auth.client_secret.clone())),
        );

        Ok(OIDCManager {
            org_id,
            client,
            config: org_auth,
        })
    }

    pub fn authorization_url(
        &self,
        redirect_url: &str,
    ) -> Result<(Url, CsrfToken, Nonce), openidconnect::url::ParseError> {
        let result = self
            .client
            .clone()
            .set_redirect_uri(RedirectUrl::new(redirect_url.to_string())?)
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new("profile".to_string()))
            .add_scope(Scope::new("email".to_string()))
            .url();
        Ok(result)
    }

    async fn map_teams<T: ConnectionTrait>(
        &self,
        conn: &T,
        user_id: PalformDatabaseID<IDAdminUser>,
        claims: &IdTokenClaims<GroupsClaim, CoreGenderClaim>,
    ) -> Result<(), TokenExchangeError> {
        let mut member_teams =
            OrganisationTeamsManager::list_member_teams(conn, self.org_id, user_id)
                .await
                .map_err(|e| {
                    TokenExchangeError::TeamMappingError(format!(
                        "list user team memberships: {}",
                        e
                    ))
                })?;

        let default_team = OrganisationTeamsManager::get_org_default_team(conn, self.org_id)
            .await
            .map_err(|e| {
                TokenExchangeError::TeamMappingError(format!("get org default team: {}", e))
            })?;

        if member_teams
            .iter()
            .find(|v| v.team_id == default_team.id)
            .is_none()
        {
            OrganisationTeamsManager::add_member_to_team(
                conn,
                default_team.id,
                user_id,
                OrganisationMemberRoleEnum::Viewer,
            )
            .await
            .map_err(|e| {
                TokenExchangeError::TeamMappingError(format!("add user to default team: {}", e))
            })?;

            member_teams.push(APIOrganisationTeamMembership {
                team_id: default_team.id,
                name: default_team.name,
                my_role: OrganisationMemberRoleEnum::Viewer,
            });
        }

        if let Some(field_name) = self.config.team_mapping_field_name.clone() {
            let field_value = claims.additional_claims().0.get(&field_name).ok_or(
                TokenExchangeError::TeamMappingError(format!(
                    "Field {} not found in user_info response",
                    field_name
                )),
            )?;

            let array = field_value
                .as_array()
                .ok_or(TokenExchangeError::TeamMappingError(format!(
                    "{} in user_info response was not an array",
                    field_name
                )))?;

            let team_mappings = OrganisationAuthTeamMappingsManager::new(self.org_id)
                .list(conn)
                .await
                .map_err(|e| {
                    TokenExchangeError::TeamMappingError(format!(
                        "list organisation team mappings: {}",
                        e.to_string()
                    ))
                })?;

            let mut teams_seen = HashSet::<PalformDatabaseID<IDTeam>>::new();
            for val in array {
                let val = val
                    .as_str()
                    .ok_or(TokenExchangeError::TeamMappingError(format!(
                        "Found non-string value in {} in user_info response",
                        field_name
                    )))?;

                let mut matching_mapping = None::<&APIOrganisationAuthTeamMapping>;
                for mapping in &team_mappings {
                    if mapping.field_value == val.to_owned() {
                        matching_mapping = Some(mapping);
                        break;
                    }
                }

                if let Some(matching_mapping) = matching_mapping {
                    teams_seen.insert(matching_mapping.team_id);

                    let existing_membership = member_teams
                        .iter()
                        .find(|e| e.team_id == matching_mapping.team_id);

                    if let Some(existing_membership) = existing_membership {
                        if matching_mapping.role == existing_membership.my_role {
                            // already a member of the team with the correct role, nothing to do
                            continue;
                        }

                        OrganisationTeamsManager::change_member_role(
                            conn,
                            matching_mapping.team_id,
                            user_id,
                            matching_mapping.role.clone(),
                        )
                        .await
                        .map_err(|e| {
                            TokenExchangeError::TeamMappingError(format!(
                                "updating team membership: {}",
                                e
                            ))
                        })?;
                    } else {
                        OrganisationTeamsManager::add_member_to_team(
                            conn,
                            matching_mapping.team_id,
                            user_id,
                            matching_mapping.role.clone(),
                        )
                        .await
                        .map_err(|e| {
                            TokenExchangeError::TeamMappingError(format!(
                                "adding user to team: {}",
                                e
                            ))
                        })?;
                    }
                }
            }

            if self.config.revoke_team_mappings == Some(true) {
                for member_team in member_teams {
                    if member_team.team_id == default_team.id {
                        // we cannot remove users from the default team
                        continue;
                    }

                    if !teams_seen.contains(&member_team.team_id) {
                        OrganisationTeamsManager::remove_from_team(
                            conn,
                            member_team.team_id,
                            user_id,
                        )
                        .await
                        .map_err(|e| {
                            TokenExchangeError::TeamMappingError(format!(
                                "remove user from team: {}",
                                e
                            ))
                        })?;
                    }
                }
            }
        }

        Ok(())
    }

    pub async fn token_exchange<T: ConnectionTrait>(
        &self,
        conn: &T,
        auth_code: String,
        nonce: String,
        redirect_url: String,
    ) -> Result<(NewAPIAuthToken, PalformDatabaseID<IDAdminUser>), TokenExchangeError> {
        let result = oidc_common_token_exchange(
            conn,
            self.client.clone(),
            auth_code,
            nonce,
            redirect_url,
            Some(self.org_id),
            None,
        )
        .await?;

        let mut user_id = result.sub_matched_user.map(|v| v.id);
        if let Some(email_matched_user) = result.email_matched_user {
            if let Some(user_org_id) = email_matched_user.org_auth_organisation_id {
                if email_matched_user
                    .org_auth_sub
                    .is_some_and(|v| v != result.sub.to_owned())
                    && user_org_id == self.org_id
                {
                    return Err(TokenExchangeError::UserConflict(
                        "Someone in your organisation already has your email address and we can't associate your account. Please get in touch with us for help."
                            .to_string(),
                    ));
                }
            }

            if user_id.is_none() && email_matched_user.org_auth_organisation_id.is_none() {
                AdminUserManager::associate_user_with_org(
                    conn,
                    email_matched_user.id,
                    self.org_id,
                    result.sub.to_owned(),
                )
                .await
                .map_err(|e| {
                    TokenExchangeError::CreateUserError(AdminUserManagementError::DBError(e))
                })?;
                user_id = Some(email_matched_user.id)
            }
        }

        let token_user_id: PalformDatabaseID<IDAdminUser>;
        if let Some(user_id) = user_id {
            token_user_id = user_id;
        } else {
            let user_display_name = result
                .user_info
                .nickname()
                .ok_or(TokenExchangeError::InvalidClaims(
                    "No name claim returned".to_string(),
                ))?
                .get(None)
                .ok_or(TokenExchangeError::InvalidClaims(
                    "Name claim for locale not found".to_string(),
                ))?;

            token_user_id = AdminUserManager::create_user_for_org(
                conn,
                user_display_name.to_string(),
                result.email,
                self.org_id,
                result.sub,
            )
            .await?;

            OrganisationMembersManager::create(conn, self.org_id, token_user_id, false)
                .await
                .map_err(|e| {
                    TokenExchangeError::CreateUserError(AdminUserManagementError::DBError(e))
                })?;
        }

        self.map_teams(conn, token_user_id, &result.raw_claims)
            .await?;
        let auth_token = TokenManager::issue_token(conn, token_user_id).await?;
        Ok((auth_token, token_user_id))
    }
}
