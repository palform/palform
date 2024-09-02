use std::ops::Deref;

use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};
use rocket::{
    http::Status,
    request::{self, FromRequest},
};
use rocket_okapi::request::OpenApiFromRequest;

use crate::into_outcome;

pub struct FromOrgId<T: FromOrgIdTrait>(pub T);

impl<T: FromOrgIdTrait> FromOrgId<T> {
    fn new(org_id: PalformDatabaseID<IDOrganisation>) -> Self {
        FromOrgId(T::new(org_id))
    }
}

impl<T: FromOrgIdTrait> Deref for FromOrgId<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait FromOrgIdTrait {
    fn new(org_id: PalformDatabaseID<IDOrganisation>) -> Self;
}

#[rocket::async_trait]
impl<'a, T: FromOrgIdTrait> FromRequest<'a> for FromOrgId<T> {
    type Error = String;
    async fn from_request(
        request: &'a request::Request<'_>,
    ) -> request::Outcome<Self, Self::Error> {
        let org_param = into_outcome!(request
            .param::<PalformDatabaseID<IDOrganisation>>(3)
            .ok_or((Status::BadRequest, "Missing org param".to_string()))
            .and_then(
                |v| v.map_err(|e| (Status::BadRequest, format!("Failed to parse org id {}", e)))
            ));

        request::Outcome::Success(Self::new(org_param))
    }
}

#[rocket::async_trait]
impl<'a, T: FromOrgIdTrait> OpenApiFromRequest<'a> for FromOrgId<T> {
    fn from_request_input(
        _gen: &mut rocket_okapi::gen::OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<rocket_okapi::request::RequestHeaderInput> {
        Ok(rocket_okapi::request::RequestHeaderInput::None)
    }
}
