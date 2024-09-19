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
</script>

<MainTitle>Choose a template to get started</MainTitle>
<TextButton
    class="mb-8"
    href={`/orgs/${$orgCtx.org.id}/forms/new`}
    on:click={navigateEvent}
>
    Or start from scratch
</TextButton>

<Router>
    <Route path="/" component={MostPopularTemplates} />
    <Route path="/categories/:categoryId" component={CategoryTemplates} />
    <Route path="/:templateId" component={FormTemplatePreview} />
</Router>
