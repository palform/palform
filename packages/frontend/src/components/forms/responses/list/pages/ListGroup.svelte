<script lang="ts">
    import type { DecryptedSubmissionSuccess } from "../../../../../data/crypto/results";
    import ListQuestion from "./ListQuestion.svelte";
    import { getFormCtx } from "../../../../../data/contexts/orgLayout";
    import {
        ctxGetGroup,
        getFormAdminContext,
        getGroupTitle,
    } from "../../../../../data/contexts/formAdmin";

    export let groupId: string;
    export let submission: DecryptedSubmissionSuccess;

    const formAdminCtx = getFormAdminContext();
    const formMetadataCtx = getFormCtx();

    $: group = ctxGetGroup(groupId);
    $: questionSubmissions = submission.questions.filter((q) => {
        const question = $formAdminCtx.questions.find(
            (e) => e.id === q.question_id
        );
        if (!question) return false;
        return question.group_id === groupId;
    });
</script>

{#if $group !== undefined && !$formMetadataCtx.one_question_per_page}
    <h2 class="text-lg mb-2 dark:text-gray-300">
        {getGroupTitle(false, $formAdminCtx, $group)}
    </h2>
{/if}

<ol class="space-y-4">
    {#each questionSubmissions as questionSubmission (questionSubmission.question_id)}
        <ListQuestion {questionSubmission} />
    {/each}
</ol>
