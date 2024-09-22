<script lang="ts">
    import type {
        APIFormTemplate,
        APIFormTemplateCategory,
    } from "@paltiverse/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import {
        navigateEvent,
        TemplateCategoryList,
        TemplateItemPreview,
    } from "@paltiverse/palform-frontend-common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import TextButton from "../../../components/TextButton.svelte";

    export let source: "top" | string;

    const orgCtx = getOrgContext();

    let categories: APIFormTemplateCategory[] | undefined = undefined;
    let categoriesLoading = true;

    APIs.formTemplates
        .formTemplatesCategoriesList()
        .then((resp) => {
            categories = resp.data;
        })
        .catch(showFailureToast)
        .finally(() => (categoriesLoading = false));

    let templates: APIFormTemplate[] | undefined = undefined;
    let templatesLoading = true;

    $: {
        templatesLoading = true;

        const request =
            source === "top"
                ? APIs.formTemplates.formTemplatesListTop()
                : APIs.formTemplates.formTemplatesList(source);

        request
            .then((resp) => {
                templates = resp.data;
            })
            .catch(showFailureToast)
            .finally(() => (templatesLoading = false));
    }
</script>

<div class="grid grid-cols-4 gap-10">
    <div>
        {#if source !== "top"}
            <TextButton
                class="mb-4"
                href={`/orgs/${$orgCtx.org.id}/forms/templates/`}
                on:click={navigateEvent}
            >
                Show all templates
            </TextButton>
        {/if}

        {#if categories}
            <TemplateCategoryList
                {categories}
                categoryURL={(id) =>
                    `/orgs/${$orgCtx.org.id}/forms/templates/categories/${id}`}
                selectedId={source === "top" ? undefined : source}
                routed
            />
        {/if}
    </div>

    <div class="col-span-3">
        <div class="grid grid-cols-3 gap-8">
            {#if templates}
                {#each templates as template (template.id)}
                    <TemplateItemPreview
                        {template}
                        link={`/orgs/${$orgCtx.org.id}/forms/templates/${template.id}`}
                        routed
                    />
                {/each}
            {/if}
        </div>
    </div>
</div>
