use std::{future::Future, pin::Pin};

use actix_web::{
    dev::Payload,
    web::{self, Data},
    FromRequest, HttpRequest,
};
use apistos::ApiSecurity;
use chrono::{DateTime, Utc};
use palform_client_common::errors::error::APIError;
use palform_entities::{fill_access_token, form, organisation, prelude::*, team};
use palform_tsid::{
    resources::{IDFillAccessToken, IDForm, IDOrganisation},
    tsid::PalformDatabaseID,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, DbErr,
    EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, Set,
};
use serde::Deserialize;

use crate::{
    api_entities::fill_token::{APIExchangedShortLink, APIFillToken},
    i18n::request::I18NManager,
    pt,
};

#[derive(ApiSecurity)]
#[openapi_security(scheme(security_type(api_key(name = "f", api_key_in = "query"))))]
pub struct APIFillAccessToken {
    pub token_id: PalformDatabaseID<IDFillAccessToken>,
    pub form_id: PalformDatabaseID<IDForm>,
}

impl FromRequest for APIFillAccessToken {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        let i18n_manager_fut = I18NManager::from_request(&req, payload);

        #[derive(Deserialize)]
        struct FillAccessTokenQuery {
            f: PalformDatabaseID<IDFillAccessToken>,
        }

        let fill_access_token_fut = web::Query::<FillAccessTokenQuery>::from_request(&req, payload);
        let req = req.clone();

        Box::pin(async move {
            let i18n_manager = i18n_manager_fut.await?;
            let fill_access_token = fill_access_token_fut.await.map_err(|e| {
                APIError::BadRequest(format!("Missing `f` parameter: {}", e.to_string()))
            })?;

            let form_id: PalformDatabaseID<IDForm> = req
                .match_info()
                .get("form_id")
                .ok_or(APIError::BadRequest("Missing form_id in route".to_string()))?
                .parse()
                .map_err(|e| APIError::BadRequest(format!("Invalid form ID: {}", e)))?;

            let db = req.app_data::<Data<DatabaseConnection>>().ok_or_else(|| {
                APIError::report_internal_error_without_error("DB not found in app data")
            })?;

            let token_data = FillAccessTokenManager::lookup(db.as_ref(), fill_access_token.f)
                .await
                .map_err(|e| APIError::report_internal_error("Lookup fill token data", e))?
                .ok_or(APIError::NotAllowed)?;

            if token_data.form_id != form_id {
                return Err(APIError::NotFound);
            }

            if let Some(expires_at) = token_data.expires_at {
                if expires_at < Utc::now() {
                    let err = APIError::BadRequest(pt!(i18n_manager, "fill_form_expired",));
                    return Err(err);
                }
            }

            Ok(APIFillAccessToken {
                token_id: token_data.id,
                form_id: token_data.form_id,
            })
        })
    }
}

pub struct FillAccessTokenManager;
impl FillAccessTokenManager {
    async fn lookup<T: ConnectionTrait>(
        conn: &T,
        id: PalformDatabaseID<IDFillAccessToken>,
    ) -> Result<Option<fill_access_token::Model>, DbErr> {
        FillAccessToken::find_by_id(id).one(conn).await
    }

    pub async fn list_for_form<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<Vec<APIFillToken>, DbErr> {
        FillAccessToken::find()
            .filter(fill_access_token::Column::FormId.eq(form_id))
            .into_model::<APIFillToken>()
            .all(conn)
            .await
    }

    pub async fn create<T: ConnectionTrait>(
        conn: &T,
        form_id: PalformDatabaseID<IDForm>,
        nickname: String,
        expires_at: Option<DateTime<Utc>>,
        short_link: Option<String>,
    ) -> Result<APIFillToken, DbErr> {
        let new_token = fill_access_token::ActiveModel {
            id: Set(PalformDatabaseID::<IDFillAccessToken>::random()),
            form_id: Set(form_id),
            expires_at: Set(expires_at.map(|v| v.fixed_offset())),
            nickname: Set(nickname),
            short_link: Set(short_link),
            ..Default::default()
        };

        let resp = new_token.insert(conn).await?;
        Ok(resp.into())
    }

    pub async fn verify_token_form<T: ConnectionTrait>(
        conn: &T,
        token_id: PalformDatabaseID<IDFillAccessToken>,
        form_id: PalformDatabaseID<IDForm>,
    ) -> Result<bool, DbErr> {
        let resp = FillAccessToken::find_by_id(token_id)
            .filter(fill_access_token::Column::FormId.eq(form_id))
            .count(conn)
            .await?;
        Ok(resp == 1)
    }

    pub async fn delete<T: ConnectionTrait>(
        conn: &T,
        id: PalformDatabaseID<IDFillAccessToken>,
    ) -> Result<(), DbErr> {
        FillAccessToken::delete_by_id(id)
            .exec(conn)
            .await
            .map(|_| ())
    }

    pub async fn short_link_is_unique<T: ConnectionTrait>(
        conn: &T,
        org_id: PalformDatabaseID<IDOrganisation>,
        subdomain: String,
    ) -> Result<bool, DbErr> {
        FillAccessToken::find()
            .join(JoinType::InnerJoin, fill_access_token::Relation::Form.def())
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .filter(
                Condition::all()
                    .add(fill_access_token::Column::ShortLink.eq(Some(subdomain)))
                    .add(team::Column::OrganisationId.eq(org_id)),
            )
            .count(conn)
            .await
            .map(|c| c == 0)
    }

    pub async fn get_short_link<T: ConnectionTrait>(
        conn: &T,
        subdomain: String,
        short_link: String,
    ) -> Result<Option<APIExchangedShortLink>, DbErr> {
        FillAccessToken::find()
            .join(JoinType::InnerJoin, fill_access_token::Relation::Form.def())
            .join(JoinType::InnerJoin, form::Relation::Team.def())
            .join(JoinType::InnerJoin, team::Relation::Organisation.def())
            .filter(
                Condition::all()
                    .add(fill_access_token::Column::ShortLink.eq(Some(short_link)))
                    .add(organisation::Column::Subdomain.eq(Some(subdomain))),
            )
            .select_only()
            .column_as(fill_access_token::Column::Id, "fill_token_id")
            .column_as(organisation::Column::Id, "org_id")
            .column_as(form::Column::Id, "form_id")
            .column(fill_access_token::Column::ExpiresAt)
            .into_model()
            .one(conn)
            .await
    }
}
