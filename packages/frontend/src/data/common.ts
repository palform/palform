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

export const urlsBase = import.meta.env.VITE_URLS_BASE as string;
export const frontendURL = import.meta.env.VITE_FRONTEND_URL as string;
export const backendURL = import.meta.env.VITE_BACKEND_URL as string;

export const baseAPIConfig = {
    basePath: backendURL,
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
                    return "Something went wrong on the server; please try again.";
                }
                if (d === "NotAllowed") {
                    return "You aren't allowed to do that.";
                }
                if (d === "NotFound") {
                    return `${resourceName} was not found`;
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
                return `Please upgrade your plan: ${d.SubscriptionLimit}`;
            }
            if (e.response.status === 403) {
                return "You aren't allowed to do that.";
            }
            if (e.response.status === 422) {
                return "Your input was not valid.";
            }
        }

        return "Failed to connect to server.";
    }
    if (e instanceof Error) {
        return e.message;
    }
    if (typeof e === "string") {
        return e;
    }

    console.error(e);
    return "Something went wrong. Please check the console.";
}
