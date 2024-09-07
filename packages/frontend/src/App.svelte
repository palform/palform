<script lang="ts">
    import { Route, Router } from "svelte-routing";
    import FillForm from "./pages/fill/FillForm.svelte";
    import ImperativeToastManager from "./components/toasts/ImperativeToastManager.svelte";
    import NewOrganisation from "./pages/orgs/NewOrganisation.svelte";
    import JoinOrganisation from "./pages/orgs/JoinOrganisation.svelte";
    import AsyncOrganisationRouter from "./pages/orgs/AsyncOrganisationRouter.svelte";
    import OrganisationSwitcher from "./pages/orgs/OrganisationSwitcher.svelte";
    import AsyncAuthRouter from "./pages/auth/AsyncAuthRouter.svelte";

    export let url = "";
</script>

<div class={`min-h-full h-full`}>
    <Router {url}>
        <Route path="/auth/*" component={AsyncAuthRouter} />

        <Route path="/orgs/new" component={NewOrganisation} />
        <Route
            path="/orgs/join/:orgId/:inviteId"
            component={JoinOrganisation}
        />
        <Route path="/orgs/:orgId/*" component={AsyncOrganisationRouter} />

        <Route path="/:fillShortLink" component={FillForm} />
        <Route path="/fill/:orgId/:formId" component={FillForm} />

        <Route path="/" component={OrganisationSwitcher} />
    </Router>

    <ImperativeToastManager />
</div>
