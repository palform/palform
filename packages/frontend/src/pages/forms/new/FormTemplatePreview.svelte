<script lang="ts">
    import type { APIFormTemplate } from "@paltiverse/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import { showFailureToast } from "../../../data/toast";
    import {
        navigateEvent,
        TemplateFramePreview,
    } from "@paltiverse/palform-frontend-common";
    import TextButton from "../../../components/TextButton.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faArrowLeft } from "@fortawesome/free-solid-svg-icons";
    import { getOrgContext } from "../../../data/contexts/orgLayout";

    export let templateId: string;
    const orgCtx = getOrgContext();

    let template: APIFormTemplate | undefined = undefined;
    let templateLoading = true;
    APIs.formTemplates
        .formTemplatesGet(templateId)
        .then((resp) => {
            template = resp.data;
        })
        .catch(showFailureToast)
        .finally(() => (templateLoading = false));

    APIs.formTemplates
        .formTemplatesReportView(templateId)
        .catch(showFailureToast);
</script>

<TextButton
    class="mb-4"
    href={`/orgs/${$orgCtx.org.id}/forms/templates/`}
    on:click={navigateEvent}
>
    <FontAwesomeIcon icon={faArrowLeft} />
    Back to all templates
</TextButton>

{#if template}
    <TemplateFramePreview
        {template}
        appBaseURL={window.location.origin}
        showMarketing={false}
        buttonLinkToAuth={false}
    />
{/if}
