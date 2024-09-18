use std::fmt::Display;

use openidconnect::{
    core::{CoreAuthenticationFlow, CoreClient, CoreProviderMetadata},
    reqwest::async_http_client,
    Client, ClientId, ClientSecret, CsrfToken, IssuerUrl, Nonce, RedirectUrl, Scope,
};
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use sea_orm::ConnectionTrait;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use url::Url;

use crate::{
    config::Config,
    entity_managers::{
        admin_users::AdminUserManager, orgs::OrganisationManager,
        social_auth_connections::SocialAuthConnectionsManager,
    },
};

use super::{
    oidc_common::{oidc_common_token_exchange, TokenExchangeError},
    tokens::{NewAPIAuthToken, TokenManager},
};

#[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, PartialEq)]
pub enum SocialAuthService {
    Google,
}

impl Display for SocialAuthService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Google => write!(f, "Google"),
        }
    }
}

#[derive(Error, Debug, Clone)]
pub enum SocialAuthError {
    #[error("Social auth service not found")]
    ServiceNotFound,
    #[error("Finding auth service metadata: {0}")]
    MetadataError(String),
}

pub struct SocialAuthManager {
    service: SocialAuthService,
    client: CoreClient,
}

impl SocialAuthManager {
    pub fn list_providers(config: &Config) -> Vec<SocialAuthService> {
        config
            .social_auth_providers
            .iter()
            .map(|e| e.service.clone())
            .collect()
    }

    pub async fn new(service: SocialAuthService, config: &Config) -> Result<Self, SocialAuthError> {
        let matching_record = config
            .social_auth_providers
            .iter()
            .find(|e| e.service == service)
            .ok_or(SocialAuthError::ServiceNotFound)?;

        let provider_metadata = CoreProviderMetadata::discover_async(
            IssuerUrl::new(matching_record.discovery_url.clone())
                .map_err(|e| SocialAuthError::MetadataError(e.to_string()))?,
            async_http_client,
        )
        .await
        .map_err(|e| SocialAuthError::MetadataError(e.to_string()))?;

        let client = Client::from_provider_metadata(
            provider_metadata,
            ClientId::new(matching_record.client_id.clone()),
            Some(ClientSecret::new(matching_record.client_secret.clone())),
        );

        Ok(Self { service, client })
    }

    pub fn authorization_url(
        &self,
        redirect_url: String,
    ) -> Result<(Url, CsrfToken, Nonce), SocialAuthError> {
        let result = self
            .client
            .clone()
            .set_redirect_uri(
                RedirectUrl::new(redirect_url.to_string())
                    .map_err(|e| SocialAuthError::MetadataError(e.to_string()))?,
            )
            .authorize_url(
                CoreAuthenticationFlow::AuthorizationCode,
                CsrfToken::new_random,
                Nonce::new_random,
            )
            .add_scope(Scope::new("email".to_string()))
            .url();
        Ok(result)
    }

    pub async fn token_exchange<T: ConnectionTrait>(
        &self,
        conn: &T,
        stripe_client: &stripe::Client,
        auth_code: String,
        nonce: String,
        redirect_url: String,
    ) -> Result<(NewAPIAuthToken, Option<PalformDatabaseID<IDOrganisation>>), TokenExchangeError>
    {
        let result = oidc_common_token_exchange(
            conn,
            self.client.clone(),
            auth_code,
            nonce,
            redirect_url,
            None,
            Some(self.service.clone()),
        )
        .await?;

        let mut token_user_id = result.sub_matched_user.map(|v| v.id);
        if let Some(email_matched_user) = result.email_matched_user {
            if token_user_id.is_none() {
                let all_connections =
                    SocialAuthConnectionsManager::list_for_user(conn, email_matched_user.id)
                        .await?;

                let is_connected = all_connections
                    .iter()
                    .find(|v| v.service == self.service.to_string())
                    .is_some();

                if !is_connected {
                    SocialAuthConnectionsManager::add_connection(
                        conn,
                        email_matched_user.id,
                        self.service.clone(),
                        result.sub,
                    )
                    .await?;

                    token_user_id = Some(email_matched_user.id);
                }
            }
        }

        let mut created_org_id = None::<PalformDatabaseID<IDOrganisation>>;
        if token_user_id.is_none() {
            let new_user_id =
                AdminUserManager::create_user_without_auth(conn, result.email).await?;
            token_user_id = Some(new_user_id);

            let new_org_id =
                OrganisationManager::create(conn, "Unnamed organisation".to_string()).await?;
            created_org_id = Some(new_org_id);

            OrganisationManager::bootstrap_new_org(conn, new_org_id, new_user_id, stripe_client)
                .await?;
        }

        let token_user_id = token_user_id.expect("token_user_id must be assigned by now");
        let auth_token = TokenManager::issue_token(conn, token_user_id).await?;
        Ok((auth_token, created_org_id))
    }
}
