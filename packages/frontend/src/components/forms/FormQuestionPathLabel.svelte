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

    interface Props {
        questionId: string;
        featureName?: string | undefined;
    }

    let { questionId, featureName = undefined }: Props = $props();

    const formMetadataCtx = getFormCtx();
    const formAdminCtx = getFormAdminContext();

    let question = $derived(ctxGetQuestion(questionId));
    let group = $derived($question ? ctxGetGroup($question.group_id) : undefined);
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
