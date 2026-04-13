<script lang="ts">
    import type {
        APIFormTemplate,
        APIFormTemplateCategory,
    } from "@palform/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import {
        TemplateCategoryList,
        TemplateItemPreview,
    } from "@palform/palform-frontend-common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { p } from "../../../router";
    import TextButton from "../../../components/TextButton.svelte";
    import { Card } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import MainTitle from "../../../layouts/MainTitle.svelte";

    interface Props {
        source: "top" | string;
    }

    let { source }: Props = $props();

    const orgCtx = getOrgContext();

    let categories: APIFormTemplateCategory[] | undefined = $state(undefined);
    let categoriesLoading = true;

    APIs.formTemplates
        .formTemplatesCategoriesList()
        .then((resp) => {
            categories = resp.data;
        })
        .catch(showFailureToast)
        .finally(() => (categoriesLoading = false));

    let templates: APIFormTemplate[] | undefined = $state(undefined);
    let templatesLoading = $state(true);

    $effect(() => {
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
    });
</script>

<MainTitle>Create a new form</MainTitle>

<div class="grid grid-cols-4 gap-10 mt-4">
    <div>
        {#if source !== "top"}
            <TextButton
                class="mb-4"
                href={p("/orgs/:orgId/forms/templates", {
                    params: { orgId: $orgCtx.org.id },
                })}
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
            />
        {/if}
    </div>

    <div class="col-span-3">
        <div class="grid grid-cols-3 gap-8">
            {#if source === "top"}
                <Card
                    class="p-4 w-full bg-primary-50 hover:bg-primary-100 border-primary-200"
                    href={p("/orgs/:orgId/forms/new", {
                        params: { orgId: $orgCtx.org.id },
                    })}
                >
                    <h5
                        class="text-gray-800 dark:text-gray-200 text-xl leading-tight font-medium"
                    >
                        Start from scratch
                    </h5>
                    <p class="mt-2 leading-tight">
                        Build a new form without a template
                    </p>
                    <p class="mt-2 leading-tight text-gray-500">
                        Get started
                        <FontAwesomeIcon icon={faArrowRight} />
                    </p>
                </Card>
            {/if}

            {#if templates}
                {#each templates as template (template.id)}
                    <TemplateItemPreview
                        {template}
                        link={`/orgs/${$orgCtx.org.id}/forms/templates/${template.id}`}
                    />
                {/each}
            {/if}
        </div>
    </div>
</div>
