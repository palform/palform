<script lang="ts">
    import { Route, Router } from "svelte-routing";
    import OrganisationHome from "./OrganisationHome.svelte";
    import FormNew from "../forms/FormNew.svelte";
    import FormAdmin from "../forms/FormAdmin.svelte";
    import OrganisationMembers from "./OrganisationMembers.svelte";
    import OrganisationInvites from "./OrganisationInvites.svelte";
    import OrganisationSidebar from "../../layouts/sidebar/OrganisationSidebar.svelte";
    import OrganisationTeams from "./teams/OrganisationTeams.svelte";
    import OrganisationTeamManage from "./teams/OrganisationTeamManage.svelte";
    import { billingEnabled } from "../../data/billing/util";
    import OrganisationBilling from "./billing/OrganisationBilling.svelte";
    import InductionResources from "../induction/InductionResources.svelte";
    import InductionBilling from "../induction/InductionBilling.svelte";
    import InductionBillingComplete from "../induction/InductionBillingComplete.svelte";
    import InductionKey from "../induction/InductionKey.svelte";
    import InductionNewMember from "../induction/InductionNewMember.svelte";
    import OrganisationAudit from "./OrganisationAudit.svelte";
    import OrganisationAuthManager from "./auth/OrganisationAuthManager.svelte";
    import OrganisationSubdomainConfig from "./subdomain/OrganisationSubdomainConfig.svelte";
    import OrganisationAuthTeamMappings from "./auth/OrganisationAuthTeamMappings.svelte";
    import OrganisationMetadata from "./OrganisationMetadata.svelte";
    import UserKeys from "./user/UserKeys.svelte";
    import UserKeyRegister from "./user/UserKeyRegister.svelte";
    import UserKeyImport from "./user/UserKeyImport.svelte";
    import UserKeyBackup from "./user/UserKeyBackup.svelte";
    import UserSettings from "./user/UserSettings.svelte";
    import OrganisationKeyManager from "./OrganisationKeyManager.svelte";

    export let orgId: string;
</script>

<OrganisationSidebar {orgId}>
    <Router>
        <Route path="/" component={OrganisationHome} />
        <Route path="/induction" component={InductionResources} />
        <Route path="/induction/billing" component={InductionBilling} />
        <Route
            path="/induction/billing-complete"
            component={InductionBillingComplete}
        />
        <Route path="/induction/key" component={InductionKey} />
        <Route path="/induction/member" component={InductionNewMember} />
        <Route path="/settings/teams" component={OrganisationTeams} />
        <Route
            path="/settings/teams/:teamId/*"
            component={OrganisationTeamManage}
        />
        <Route path="/settings/members" component={OrganisationMembers} />
        <Route
            path="/settings/members/invite"
            component={OrganisationInvites}
        />
        <Route path="/settings/keys" component={OrganisationKeyManager} />
        <Route path="/settings/audit" component={OrganisationAudit} />
        <Route
            path="/settings/subdomain"
            component={OrganisationSubdomainConfig}
        />
        <Route path="/settings/auth" component={OrganisationAuthManager} />
        <Route
            path="/settings/auth/mappings"
            component={OrganisationAuthTeamMappings}
        />
        {#if billingEnabled}
            <Route path="/settings/billing" component={OrganisationBilling} />
        {/if}
        <Route path="/settings/org" component={OrganisationMetadata} />
        <Route path="/user/keys" component={UserKeys} />
        <Route path="/user/keys/register" component={UserKeyRegister} />
        <Route path="/user/keys/import" component={UserKeyImport} />
        <Route path="/user/keys/:keyId/backup" component={UserKeyBackup} />
        <Route path="/user/settings" component={UserSettings} />
        <Route path="/forms/new" component={FormNew} />
        <Route path="/forms/new/:initialTeamId" component={FormNew} />
        <Route path="/forms/:formId/*" component={FormAdmin} />
    </Router>
</OrganisationSidebar>
