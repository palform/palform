<script lang="ts">
    import type { DecryptedSubmissionSuccess } from "../../../../../data/crypto/results";
    import {
        ctxGetGroup,
        getGroupTitle,
        getResponsesContext,
    } from "../../../../../data/contexts/results";
    import ListQuestion from "./ListQuestion.svelte";
    import { getFormCtx } from "../../../../../data/contexts/orgLayout";

    export let groupId: string;
    export let submission: DecryptedSubmissionSuccess;

    const formCtx = getResponsesContext();
    const formMetadataCtx = getFormCtx();
    $: group = ctxGetGroup(groupId);
    $: questionSubmissions = submission.questions.filter((q) => {
        const question = $formCtx.questions.find((e) => e.id === q.question_id);
        if (!question) return false;
        return question.group_id === groupId;
    });
</script>

{#if $group !== undefined && !$formMetadataCtx.one_question_per_page}
    <h2 class="text-lg mb-2 dark:text-gray-300">
        {getGroupTitle(false, $formCtx, $group)}
    </h2>
{/if}

<ol class="space-y-4">
    {#each questionSubmissions as questionSubmission (questionSubmission.question_id)}
        <ListQuestion {questionSubmission} />
    {/each}
</ol>
