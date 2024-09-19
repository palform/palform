import {
    AdminUsersApi,
    AuditLogsApi,
    AuthenticationApi,
    BillingCustomersApi,
    BillingEntitlementsApi,
    BillingInvoicesApi,
    BillingPlansApi,
    Class2FAMethodsApi,
    Configuration,
    CountryMetadataApi,
    FillAccessTokensApi,
    FormBrandingsApi,
    FormTemplatesApi,
    FormsApi,
    InductionApi,
    OrganisationAuthenticationConfigurationApi,
    OrganisationAuthenticationTeamMappingsApi,
    OrganisationInvitesApi,
    OrganisationKeysApi,
    OrganisationMembersApi,
    OrganisationTeamMembersApi,
    OrganisationTeamsApi,
    OrganisationsApi,
    PasswordResetsApi,
    QuestionGroupsApi,
    QuestionsApi,
    SubmissionAssetApi,
    SubmissionsApi,
    TeamAssetsApi,
    UserKeysApi,
    type APIError,
    type ConfigurationParameters,
} from "@paltiverse/palform-typescript-openapi";
import { apiWithAuth } from "./auth";
import { AxiosError } from "axios";
import { is_api_error_js } from "@paltiverse/palform-client-common";
import { i18nAcceptLanguage, t } from "./contexts/i18n";

export const urlsBase = import.meta.env.VITE_URLS_BASE as string;
export const frontendURL = import.meta.env.VITE_FRONTEND_URL as string;
export const backendURL = import.meta.env.VITE_BACKEND_URL as string;

export const baseAPIConfig = {
    basePath: backendURL,
    baseOptions: {
        headers: {
            "Accept-Language": i18nAcceptLanguage(),
        },
    },
} satisfies ConfigurationParameters;

export const APIs = {
    adminUsers: () => apiWithAuth(AdminUsersApi, baseAPIConfig),
    auth: new AuthenticationApi(new Configuration(baseAPIConfig)),
    authWithToken: () => apiWithAuth(AuthenticationApi, baseAPIConfig),
    secondFactors: () => apiWithAuth(Class2FAMethodsApi, baseAPIConfig),
    passwordReset: new PasswordResetsApi(new Configuration(baseAPIConfig)),
    orgs: () => apiWithAuth(OrganisationsApi, baseAPIConfig),
    orgsNoAuth: new OrganisationsApi(new Configuration(baseAPIConfig)),
    orgAuthConfig: () =>
        apiWithAuth(OrganisationAuthenticationConfigurationApi, baseAPIConfig),
    orgAuthTeamMappings: () =>
        apiWithAuth(OrganisationAuthenticationTeamMappingsApi, baseAPIConfig),
    orgMembers: () => apiWithAuth(OrganisationMembersApi, baseAPIConfig),
    orgInvites: () => apiWithAuth(OrganisationInvitesApi, baseAPIConfig),
    orgTeams: () => apiWithAuth(OrganisationTeamsApi, baseAPIConfig),
    orgTeamMembers: () =>
        apiWithAuth(OrganisationTeamMembersApi, baseAPIConfig),
    formBrandings: () => apiWithAuth(FormBrandingsApi, baseAPIConfig),
    teamAssets: () => apiWithAuth(TeamAssetsApi, baseAPIConfig),
    forms: () => apiWithAuth(FormsApi, baseAPIConfig),
    formTemplates: new FormTemplatesApi(new Configuration(baseAPIConfig)),
    submissions: () => apiWithAuth(SubmissionsApi, baseAPIConfig),
    submissionAssets: () => apiWithAuth(SubmissionAssetApi, baseAPIConfig),
    questions: () => apiWithAuth(QuestionsApi, baseAPIConfig),
    questionGroups: () => apiWithAuth(QuestionGroupsApi, baseAPIConfig),
    keys: () => apiWithAuth(UserKeysApi, baseAPIConfig),
    orgKeys: () => apiWithAuth(OrganisationKeysApi, baseAPIConfig),
    fillTokens: () => apiWithAuth(FillAccessTokensApi, baseAPIConfig),
    fill: (fillAccessToken: string) => {
        const config = new Configuration({
            ...baseAPIConfig,
            apiKey: (name) => (name === "f" ? fillAccessToken : ""),
        });
        return {
            forms: new FormsApi(config),
            teamAssets: new TeamAssetsApi(config),
        };
    },
    formsNoAuth: new FormsApi(new Configuration(baseAPIConfig)),
    induction: () => apiWithAuth(InductionApi, baseAPIConfig),
    billingPlans: () => apiWithAuth(BillingPlansApi, baseAPIConfig),
    billingCustomers: () => apiWithAuth(BillingCustomersApi, baseAPIConfig),
    billingInvoices: () => apiWithAuth(BillingInvoicesApi, baseAPIConfig),
    billingEntitlements: () =>
        apiWithAuth(BillingEntitlementsApi, baseAPIConfig),
    audit: () => apiWithAuth(AuditLogsApi, baseAPIConfig),
    countries: new CountryMetadataApi(new Configuration(baseAPIConfig)),
};

// biome-ignore lint/suspicious/noExplicitAny: catch has an any error type
function isAPIError(e: any): e is APIError {
    return is_api_error_js(e);
}

// biome-ignore lint/suspicious/noExplicitAny: catch has an any error type
export function humaniseAPIError(e: any, resourceName = "That") {
    if (e instanceof AxiosError) {
        if (e.response) {
            const d = e.response.data;
            if (isAPIError(d)) {
                if (d === "Internal") {
                    return t("error_internal");
                }
                if (d === "NotAllowed") {
                    return t("error_not_allowed");
                }
                if (d === "NotFound") {
                    return (
                        t("error_not_found_1") +
                        resourceName +
                        t("error_not_found_2")
                    );
                }
                if (typeof d === "string") {
                    return d;
                }
                if ("BadRequest" in d) {
                    return d.BadRequest;
                }
                if ("ValidationError" in d) {
                    return d.ValidationError.replaceAll("\n", ", ");
                }
                if ("CaptchaError" in d) {
                    return `Captcha: ${d.CaptchaError}`;
                }

                return t("error_upgrade") + ": " + d.SubscriptionLimit;
            }
            if (e.response.status === 403) {
                return t("error_not_allowed");
            }
            if (e.response.status === 422) {
                return t("error_invalid_input");
            }
        }

        return t("error_connection");
    }
    if (e instanceof Error) {
        return e.message;
    }
    if (typeof e === "string") {
        return e;
    }

    console.error(e);
    return t("error_console");
}
