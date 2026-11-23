import {
    createRouter,
    type LazyRouteComponent,
    type Navigation,
} from "sv-router";
import FillForm from "./pages/fill/FillForm.svelte";

type AdminPages = typeof import("./adminPages");
type RouteComponent = Awaited<ReturnType<LazyRouteComponent>>["default"];
function admin(name: keyof AdminPages): LazyRouteComponent {
    return () =>
        import("./adminPages").then((m) => ({
            default: m[name] as RouteComponent,
        }));
}

const routerApi = createRouter({
    "/": admin("OrganisationSwitcher"),
    "/orgs/new": admin("NewOrganisation"),
    "/orgs/join/:orgId/:inviteId": admin("JoinOrganisation"),
    "/orgs/:orgId": {
        layout: admin("OrganisationLayout"),
        "/": admin("OrganisationHome"),
        "/induction": admin("InductionResources"),
        "/induction/billing": admin("InductionBilling"),
        "/induction/billing-complete": admin("InductionBillingComplete"),
        "/induction/key": admin("InductionKey"),
        "/induction/member": admin("InductionNewMember"),
        "/settings/teams": admin("OrganisationTeams"),
        "/settings/teams/:teamId": {
            layout: admin("OrganisationTeamManage"),
            "/members": admin("TeamMembers"),
            "/brandings": admin("TeamBrandings"),
            "/settings": admin("TeamSettings"),
        },
        "/settings/members": admin("OrganisationMembers"),
        "/settings/members/invite": admin("OrganisationInvites"),
        "/settings/keys": admin("OrganisationKeyManager"),
        "/settings/audit": admin("OrganisationAudit"),
        "/settings/subdomain": admin("OrganisationSubdomainConfig"),
        "/settings/auth": admin("OrganisationAuthManager"),
        "/settings/auth/mappings": admin("OrganisationAuthTeamMappings"),
        "/settings/billing": admin("OrganisationBillingEntry"),
        "/settings/org": admin("OrganisationMetadata"),
        "/settings/service-accounts": admin("OrganisationServiceAccounts"),
        "/user/keys": admin("UserKeys"),
        "/user/keys/register": admin("UserKeyRegister"),
        "/user/keys/import": admin("UserKeyImport"),
        "/user/keys/:keyId/backup": admin("UserKeyBackup"),
        "/user/settings": admin("UserSettings"),
        "/forms/templates": {
            "/": admin("MostPopularTemplates"),
            "/categories/:categoryId": admin("CategoryTemplates"),
            "/:templateId": admin("FormTemplatePreview"),
        },
        "/forms/new": admin("FormNew"),
        "/forms/new/:initialTeamId": admin("FormNew"),
        "/forms/:formId": {
            layout: admin("FormAdmin"),
            "/overview": admin("FormResponseOverview"),
            "/responses": admin("FormResponseList"),
            "/edit": admin("FormEditor"),
            "/tokens": admin("FormTokens"),
            "/export": admin("FormExport"),
            "/settings": admin("FormSettings"),
        },
    },
    "/auth/login": admin("Login"),
    "/auth/callback": admin("Callback"),
    "/auth/social/:providerName/callback": admin("SocialCallback"),
    "/auth/signup": admin("Signup"),
    "/auth/verify/:verificationId": admin("VerifyEmailCallback"),
    "/auth/reset/password": admin("StartPasswordReset"),
    "/auth/reset/password/:verificationId": admin("ResetPassword"),
    "/fill/:orgId/:formId": FillForm,
    "/:fillShortLink": FillForm,
});

export const p = routerApi.p;
export const isActive = routerApi.isActive;
export const route = routerApi.route;
export const typedNavigate = routerApi.navigate;

/** Same URLs as before; wrapper avoids sv-router's strict literal route typing for dynamic paths. */
export function navigate(
    path: string | number,
    options?: {
        replace?: boolean;
        search?: unknown;
        state?: unknown;
        hash?: string;
        scrollToTop?: ScrollBehavior | false;
        viewTransition?: boolean;
        params?: Record<string, string>;
    }
): Promise<Navigation> {
    return routerApi.navigate(path as never, options as never);
}
