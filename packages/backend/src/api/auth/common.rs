use palform_tsid::{
    resources::{IDAdminUserSecondAuthenticationFactorSession, IDOrganisation},
    tsid::PalformDatabaseID,
};
use rocket_okapi::okapi::schemars::{self, JsonSchema};
use serde::Serialize;
use webauthn_rs::prelude::RequestChallengeResponse;

use crate::auth::tokens::NewAPIAuthToken;

#[derive(Serialize, JsonSchema)]
pub enum SignInResponse {
    Done {
        token: NewAPIAuthToken,
        new_org_id: Option<PalformDatabaseID<IDOrganisation>>,
    },
    SecondFactorRequired {
        session_id: PalformDatabaseID<IDAdminUserSecondAuthenticationFactorSession>,
        #[schemars(with = "Option<String>")]
        rcr: Option<RequestChallengeResponse>,
        totp: bool,
        new_org_id: Option<PalformDatabaseID<IDOrganisation>>,
    },
}
