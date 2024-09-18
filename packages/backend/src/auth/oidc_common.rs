use openidconnect::{
    reqwest::async_http_client, AdditionalClaims, AuthDisplay, AuthPrompt, AuthorizationCode,
    Client, ErrorResponse, GenderClaim, IdTokenClaims, JsonWebKey, JsonWebKeyType, JsonWebKeyUse,
    JweContentEncryptionAlgorithm, JwsSigningAlgorithm, Nonce, RedirectUrl, RevocableToken,
    TokenIntrospectionResponse, TokenResponse, TokenType, UserInfoClaims,
};
use palform_entities::admin_user;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use sea_orm::{ConnectionTrait, DbErr};
use thiserror::Error;

use crate::entity_managers::{admin_users::{AdminUserManagementError, AdminUserManager}, orgs::BootstrapOrgError};

use super::{social::SocialAuthService, tokens::IssueTokenError};

#[derive(Debug, Error)]
pub enum TokenExchangeError {
    #[error("Parse URL: {0}")]
    ParseURL(#[from] openidconnect::url::ParseError),
    #[error("Exchange: {0}")]
    ExchangeError(String),
    #[error("Server did not return ID Token")]
    NoIDToken,
    #[error("Invalid claims returned by server: {0}")]
    InvalidClaims(String),
    #[error("Database error: {0}")]
    DBError(#[from] DbErr),
    #[error("Issuing token: {0}")]
    IssueToken(#[from] IssueTokenError),
    #[error("Requesting user info: {0}")]
    UserInfoError(String),
    #[error("Creating user: {0}")]
    CreateUserError(#[from] AdminUserManagementError),
    #[error("Creating default org: {0}")]
    CreateOrgError(#[from] BootstrapOrgError),
    #[error("Mapping user into teams: {0}")]
    TeamMappingError(String),
    #[error("Conflict with existing user: {0}")]
    UserConflict(String),
}

pub(super) struct OIDCTokenExchangeResult<AC, GC>
where
    AC: AdditionalClaims + Clone,
    GC: GenderClaim + Clone,
{
    pub(super) sub: String,
    pub(super) email: String,
    pub(super) raw_claims: IdTokenClaims<AC, GC>,
    pub(super) user_info: UserInfoClaims<AC, GC>,

    /// The user found in the database based on the email address returned from the OIDC
    /// provider. If this is [Some] and [Self::sub_matched_user] is [None], we have a locally
    /// stored user that's not yet associated with the OIDC provider but can safely be
    /// associated.
    pub(super) email_matched_user: Option<admin_user::Model>,
    /// The user found in the database already associated with the sub ID returned from the
    /// OIDC provider. If this is [Some] and [Self::email_matched_user] is [None], the OIDC
    /// provider is suggesting an email for the user that's different to our locally stored
    /// email.
    pub(super) sub_matched_user: Option<admin_user::Model>,
}

pub(super) async fn oidc_common_token_exchange<
    CO,
    AC,
    AD,
    GC,
    JE,
    JS,
    JT,
    JU,
    K,
    P,
    TE,
    TR,
    TT,
    TIR,
    RT,
    TRE,
>(
    conn: &CO,
    client: Client<AC, AD, GC, JE, JS, JT, JU, K, P, TE, TR, TT, TIR, RT, TRE>,
    auth_code: String,
    nonce: String,
    redirect_url: String,
    org_id: Option<PalformDatabaseID<IDOrganisation>>,
    service: Option<SocialAuthService>,
) -> Result<OIDCTokenExchangeResult<AC, GC>, TokenExchangeError>
where
    CO: ConnectionTrait,
    AC: AdditionalClaims + Clone,
    AD: AuthDisplay + Clone,
    GC: GenderClaim + Clone,
    JE: JweContentEncryptionAlgorithm<JT> + Clone,
    JS: JwsSigningAlgorithm<JT> + Clone,
    JT: JsonWebKeyType + Clone,
    JU: JsonWebKeyUse + Clone,
    K: JsonWebKey<JS, JT, JU> + Clone,
    P: AuthPrompt + Clone,
    TE: ErrorResponse + 'static + Clone,
    TR: TokenResponse<AC, GC, JE, JS, JT, TT> + Clone,
    TT: TokenType + 'static + Clone,
    TIR: TokenIntrospectionResponse<TT> + Clone,
    RT: RevocableToken + Clone,
    TRE: ErrorResponse + 'static + Clone,
{
    let token_resp = client
        .clone()
        .set_redirect_uri(RedirectUrl::new(redirect_url.to_string())?)
        .exchange_code(AuthorizationCode::new(auth_code))
        .request_async(async_http_client)
        .await
        .map_err(|e| TokenExchangeError::ExchangeError(e.to_string()))?;

    let id_token = token_resp.id_token().ok_or(TokenExchangeError::NoIDToken)?;
    let id_token_claims = id_token
        .claims(&client.id_token_verifier(), &Nonce::new(nonce))
        .map_err(|e| TokenExchangeError::InvalidClaims(e.to_string()))?;

    let string_sub_id = id_token_claims.subject().as_str();
    let sub_matched_user =
        AdminUserManager::get_user_by_sub(conn, org_id, service, string_sub_id.to_string())
            .await
            .map_err(|e| TokenExchangeError::DBError(e))?;

    let user_info: UserInfoClaims<AC, GC> = client
        .user_info(
            token_resp.access_token().to_owned(),
            Some(id_token_claims.subject().to_owned()),
        )
        .map_err(|e| TokenExchangeError::UserInfoError(e.to_string()))?
        .request_async(async_http_client)
        .await
        .map_err(|e| TokenExchangeError::UserInfoError(e.to_string()))?;

    let user_email = user_info.email().ok_or(TokenExchangeError::InvalidClaims(
        "No email claim returned".to_string(),
    ))?;

    let email_matched_user = AdminUserManager::get_user_by_email(conn, user_email.to_string())
        .await
        .map_err(|e| {
            TokenExchangeError::UserInfoError(format!(
                "check for existing user with matching email: {}",
                e
            ))
        })?;

    Ok(OIDCTokenExchangeResult {
        sub: string_sub_id.to_owned(),
        email: user_email.to_string(),
        raw_claims: id_token_claims.clone(),
        user_info,
        email_matched_user,
        sub_matched_user,
    })
}
