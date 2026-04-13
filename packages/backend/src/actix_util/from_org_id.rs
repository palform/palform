use std::{future::Future, ops::Deref, pin::Pin};

use actix_web::{dev::Payload, FromRequest, HttpRequest};
use apistos::ApiComponent;
use palform_client_common::errors::error::APIError;
use palform_tsid::{resources::IDOrganisation, tsid::PalformDatabaseID};

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

impl<T: FromOrgIdTrait> FromRequest for FromOrgId<T> {
    type Error = APIError;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let req = req.clone();
        Box::pin(async move {
            let org_param: PalformDatabaseID<IDOrganisation> = req
                .match_info()
                .get("org_id")
                .ok_or(APIError::BadRequest("Missing org ID".to_string()))?
                .parse::<PalformDatabaseID<IDOrganisation>>()
                .map_err(|e| APIError::BadRequest(format!("Invalid org ID: {}", e.to_string())))?;

            Ok(Self::new(org_param))
        })
    }
}

impl<T> ApiComponent for FromOrgId<T>
where
    T: FromOrgIdTrait,
{
    fn child_schemas() -> Vec<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        Vec::default()
    }
    fn schema() -> Option<(String, apistos::reference_or::ReferenceOr<apistos::Schema>)> {
        None
    }
}
