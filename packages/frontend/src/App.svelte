<script lang="ts">
    import { Route, Router } from "svelte-routing";
    import FillForm from "./pages/fill/FillForm.svelte";
    import ImperativeToastManager from "./components/toasts/ImperativeToastManager.svelte";
    import NewOrganisation from "./pages/orgs/NewOrganisation.svelte";
    import JoinOrganisation from "./pages/orgs/JoinOrganisation.svelte";
    import AsyncOrganisationRouter from "./pages/orgs/AsyncOrganisationRouter.svelte";
    import OrganisationSwitcher from "./pages/orgs/OrganisationSwitcher.svelte";
    import AsyncAuthRouter from "./pages/auth/AsyncAuthRouter.svelte";
    import { isInFrame } from "./data/util/iframe";

    export let url = "";
    const isFrame = isInFrame();
</script>

<div class={`min-h-full ${isFrame ? "" : "bg-slate-50/50 dark:bg-slate-900"}`}>
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
