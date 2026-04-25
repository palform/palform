import { createRouter, type Navigation } from "sv-router";
import OrganisationSwitcher from "./pages/orgs/OrganisationSwitcher.svelte";
import NewOrganisation from "./pages/orgs/NewOrganisation.svelte";
import JoinOrganisation from "./pages/orgs/JoinOrganisation.svelte";
import FillForm from "./pages/fill/FillForm.svelte";
import OrganisationLayout from "./layouts/OrganisationLayout.svelte";
import FormAdmin from "./pages/forms/FormAdmin.svelte";
import OrganisationTeamManage from "./pages/orgs/teams/OrganisationTeamManage.svelte";

const routerApi = createRouter({
    "/": OrganisationSwitcher,
    "/orgs/new": NewOrganisation,
    "/orgs/join/:orgId/:inviteId": JoinOrganisation,
    "/orgs/:orgId": {
        layout: OrganisationLayout,
        "/": () => import("./pages/orgs/OrganisationHome.svelte"),
        "/induction": () =>
            import("./pages/induction/InductionResources.svelte"),
        "/induction/billing": () =>
            import("./pages/induction/InductionBilling.svelte"),
        "/induction/billing-complete": () =>
            import("./pages/induction/InductionBillingComplete.svelte"),
        "/induction/key": () => import("./pages/induction/InductionKey.svelte"),
        "/induction/member": () =>
            import("./pages/induction/InductionNewMember.svelte"),
        "/settings/teams": () =>
            import("./pages/orgs/teams/OrganisationTeams.svelte"),
        "/settings/teams/:teamId": {
            layout: OrganisationTeamManage,
            "/members": () => import("./pages/orgs/teams/TeamMembers.svelte"),
            "/brandings": () =>
                import("./pages/orgs/teams/TeamBrandings.svelte"),
            "/settings": () => import("./pages/orgs/teams/TeamSettings.svelte"),
        },
        "/settings/members": () =>
            import("./pages/orgs/OrganisationMembers.svelte"),
        "/settings/members/invite": () =>
            import("./pages/orgs/OrganisationInvites.svelte"),
        "/settings/keys": () =>
            import("./pages/orgs/OrganisationKeyManager.svelte"),
        "/settings/audit": () =>
            import("./pages/orgs/OrganisationAudit.svelte"),
        "/settings/subdomain": () =>
            import("./pages/orgs/subdomain/OrganisationSubdomainConfig.svelte"),
        "/settings/auth": () =>
            import("./pages/orgs/auth/OrganisationAuthManager.svelte"),
        "/settings/auth/mappings": () =>
            import("./pages/orgs/auth/OrganisationAuthTeamMappings.svelte"),
        "/settings/billing": () =>
            import("./pages/orgs/billing/OrganisationBillingEntry.svelte"),
        "/settings/org": () =>
            import("./pages/orgs/OrganisationMetadata.svelte"),
        "/user/keys": () => import("./pages/orgs/user/UserKeys.svelte"),
        "/user/keys/register": () =>
            import("./pages/orgs/user/UserKeyRegister.svelte"),
        "/user/keys/import": () =>
            import("./pages/orgs/user/UserKeyImport.svelte"),
        "/user/keys/:keyId/backup": () =>
            import("./pages/orgs/user/UserKeyBackup.svelte"),
        "/user/settings": () => import("./pages/orgs/user/UserSettings.svelte"),
        "/forms/templates": {
            "/": () => import("./pages/forms/new/MostPopularTemplates.svelte"),
            "/categories/:categoryId": () =>
                import("./pages/forms/new/CategoryTemplates.svelte"),
            "/:templateId": () =>
                import("./pages/forms/new/FormTemplatePreview.svelte"),
        },
        "/forms/new": () => import("./pages/forms/FormNew.svelte"),
        "/forms/new/:initialTeamId": () =>
            import("./pages/forms/FormNew.svelte"),
        "/forms/:formId": {
            layout: FormAdmin,
            "/overview": () =>
                import("./components/forms/responses/overview/FormResponseOverview.svelte"),
            "/responses": () =>
                import("./components/forms/responses/list/FormResponseList.svelte"),
            "/edit": () => import("./pages/forms/FormEditor.svelte"),
            "/tokens": () => import("./pages/forms/FormTokens.svelte"),
            "/export": () => import("./pages/forms/FormExport.svelte"),
            "/settings": () => import("./pages/forms/FormSettings.svelte"),
        },
    },
    "/auth/login": () => import("./pages/auth/Login.svelte"),
    "/auth/callback": () => import("./pages/auth/Callback.svelte"),
    "/auth/social/:providerName/callback": () =>
        import("./pages/auth/SocialCallback.svelte"),
    "/auth/signup": () => import("./pages/auth/Signup.svelte"),
    "/auth/verify/:verificationId": () =>
        import("./pages/auth/VerifyEmailCallback.svelte"),
    "/auth/reset/password": () =>
        import("./pages/auth/StartPasswordReset.svelte"),
    "/auth/reset/password/:verificationId": () =>
        import("./pages/auth/ResetPassword.svelte"),
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
