<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faChevronRight } from "@fortawesome/free-solid-svg-icons";
    import {
        ctxGetGroup,
        ctxGetQuestion,
        getFormAdminContext,
        getGroupTitle,
    } from "../../data/contexts/formAdmin";
    import { getFormCtx } from "../../data/contexts/orgLayout";

    export let questionId: string;
    export let featureName: string | undefined = undefined;

    const formMetadataCtx = getFormCtx();
    const formAdminCtx = getFormAdminContext();

    $: question = ctxGetQuestion(questionId);
    $: group = $question ? ctxGetGroup($question.group_id) : undefined;
</script>

{#if $question && $group}
    {getGroupTitle(
        $formMetadataCtx.one_question_per_page,
        $formAdminCtx,
        $group
    )}

    <FontAwesomeIcon icon={faChevronRight} class="me-1 ms-1" />
    <strong>{$question.title}</strong>

    {#if featureName}
        <FontAwesomeIcon icon={faChevronRight} class="me-1 ms-1" />
        {featureName}
    {/if}
{/if}
