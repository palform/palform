use chrono::{NaiveDateTime, Utc};
use palform_entities::{fill_access_token, form, organisation, prelude::*, team};
use palform_tsid::{resources::{IDFillAccessToken, IDForm, IDOrganisation}, tsid::PalformDatabaseID};
use rocket::{
    http::Status,
    request::{self, FromRequest},
};
use rocket_okapi::{
    okapi::openapi3::{Object, SecurityRequirement, SecurityScheme, SecuritySchemeData},
    request::{OpenApiFromRequest, RequestHeaderInput},
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, DbErr,
    EntityTrait, JoinType, PaginatorTrait, QueryFilter, QuerySelect, RelationTrait, Set,
};

use crate::api_entities::fill_token::{APIExchangedShortLink, APIFillToken};

pub struct APIFillAccessToken {
    pub token_id: PalformDatabaseID<IDFillAccessToken>,
    pub form_id: PalformDatabaseID<IDForm>,
}

#[rocket::async_trait]
impl<'a> FromRequest<'a> for APIFillAccessToken {
    type Error = String;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        async fn do_authentication<'b>(
            request: &'b request::Request<'_>,
        ) -> Result<APIFillAccessToken, (Status, String)> {
            let fill_access_token = request
                .query_value::<PalformDatabaseID<IDFillAccessToken>>("f")
                .ok_or((Status::BadRequest, "Missing `f` parameter".to_string()))?
                .map_err(|e| {
                    (
                        Status::BadRequest,
                        format!("Failed to parse fill token: {}", e.to_string()),
                    )
                })?;

            let form_id = request
                .param::<PalformDatabaseID<IDForm>>(4)
                .ok_or((
                    Status::BadRequest,
                    "Wrong number of path segments".to_string(),
                ))?
                .map_err(|e| {
                    (
                        Status::BadRequest,
                        format!("Failed to parse form_id segment: {}", e.to_string()),
                    )
                })?;

            let db = request.rocket().state::<DatabaseConnection>().ok_or((
                Status::InternalServerError,
                "DB not found in state".to_string(),
            ))?;

            let token_data = FillAccessTokenManager::lookup(db, fill_access_token)
                .await
                .map_err(|e| {
                    (
                        Status::InternalServerError,
                        format!("Lookup fill token data: {}", e.to_string()),
                    )
                })?
                .ok_or((Status::Forbidden, "Token not found".to_string()))?;

            if token_data.form_id != form_id {
                return Err((Status::Forbidden, "Token not found".to_string()));
            }

            if let Some(expires_at) = token_data.expires_at {
                if expires_at < Utc::now().naive_utc() {
                    return Err((Status::Forbidden, "Token expired".to_string()));
                }
            }

            Ok(APIFillAccessToken {
                token_id: token_data.id,
                form_id: token_data.form_id,
            })
        }

        let result = do_authentication(request).await;
        match result {
            Err(e) => request::Outcome::Error(e),
            Ok(d) => request::Outcome::Success(d),
        }
    }
}

impl<'a> OpenApiFromRequest<'a> for APIFillAccessToken {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        let security_scheme = SecurityScheme {
            description: Some("Requires a Fill Access Token (usually provided by the form author as part of a link) to view and submit a form".to_owned()),
            data: SecuritySchemeData::ApiKey { name: "f".to_owned(), location: "query".to_owned() },
            extensions: Object::default()
        };

        let mut security_req = SecurityRequirement::new();
        security_req.insert("APIFillAccessToken".to_owned(), Vec::new());
        Ok(RequestHeaderInput::Security(
            "APIFillAccessToken".to_owned(),
            security_scheme,
            security_req,
        ))
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
        expires_at: Option<NaiveDateTime>,
        short_link: Option<String>,
    ) -> Result<APIFillToken, DbErr> {
        let new_token = fill_access_token::ActiveModel {
            id: Set(PalformDatabaseID::<IDFillAccessToken>::random()),
            form_id: Set(form_id),
            expires_at: Set(expires_at),
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

    pub async fn delete<T: ConnectionTrait>(conn: &T, id: PalformDatabaseID<IDFillAccessToken>) -> Result<(), DbErr> {
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
