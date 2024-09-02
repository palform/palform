<script lang="ts">
    import type { DecryptedSubmissionSuccess } from "../../../../data/crypto/results";
    import {
        ctxGetGroup,
        getGroupTitle,
        getResponsesContext,
    } from "../../../../data/contexts/results";
    import ListQuestion from "./ListQuestion.svelte";

    export let groupId: string;
    export let submission: DecryptedSubmissionSuccess;

    const formCtx = getResponsesContext();
    $: group = ctxGetGroup(groupId);
    $: questionSubmissions = submission.questions.filter((q) => {
        const question = $formCtx.questions.find((e) => e.id === q.question_id);
        if (!question) return false;
        return question.group_id === groupId;
    });
</script>

{#if $group !== undefined}
    <h2 class="text-lg mb-2 dark:text-gray-300">
        {getGroupTitle($group)}
    </h2>
{/if}

<ol class="space-y-4">
    {#each questionSubmissions as questionSubmission (questionSubmission.question_id)}
        <ListQuestion {questionSubmission} />
    {/each}
</ol>
