use prefixed_tsid::id_resource_type;

pub use prefixed_tsid::resources::IDUnknown;
pub use prefixed_tsid::resources::TSIDResource;

id_resource_type!(IDAdminPublicKey, "admin_pk");
id_resource_type!(IDAdminUser, "user");
id_resource_type!(IDAdminUserEmailVerification, "user_email_verification");
id_resource_type!(IDAdminUserSecondAuthenticationFactor, "user_2fa");
id_resource_type!(
    IDAdminUserSecondAuthenticationFactorSession,
    "user_2fa_sess"
);
id_resource_type!(IDAuditLogEntry, "aul");
id_resource_type!(IDAuthToken, "au");
id_resource_type!(IDFillAccessToken, "fat");
id_resource_type!(IDForm, "form");
id_resource_type!(IDFormBranding, "brand");
id_resource_type!(IDOrganisation, "org");
id_resource_type!(IDOrganisationAuthConfig, "org_auth_conf");
id_resource_type!(IDOrganisationAuthTeamMapping, "org_auth_team_map");
id_resource_type!(IDOrganisationInvite, "org_invite");
id_resource_type!(IDQuestion, "qu");
id_resource_type!(IDQuestionGroup, "qg");
id_resource_type!(IDSubmission, "sub");
id_resource_type!(IDSubmissionFile, "subf");
id_resource_type!(IDTeam, "team");
id_resource_type!(IDTeamAsset, "tas");
