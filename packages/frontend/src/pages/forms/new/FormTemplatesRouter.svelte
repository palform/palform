<script lang="ts">
    import { Route, Router } from "svelte-routing";
    import MainTitle from "../../../layouts/MainTitle.svelte";
    import MostPopularTemplates from "./MostPopularTemplates.svelte";
    import FormTemplatePreview from "./FormTemplatePreview.svelte";
    import CategoryTemplates from "./CategoryTemplates.svelte";
    import TextButton from "../../../components/TextButton.svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";

    const orgCtx = getOrgContext();

    const params = new URLSearchParams(window.location.search);
    const teamId = params.get("team");
</script>

<MainTitle>Choose a template to get started</MainTitle>
<TextButton
    class="mb-8"
    href={`/orgs/${$orgCtx.org.id}/forms/new/${teamId ?? ""}`}
    on:click={navigateEvent}
>
    Or start from scratch
</TextButton>

<Router>
    <Route path="/" component={MostPopularTemplates} />
    <Route path="/categories/:categoryId" component={CategoryTemplates} />
    <Route path="/:templateId" let:params>
        <FormTemplatePreview templateId={params.templateId} {teamId} />
    </Route>
</Router>
