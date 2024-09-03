<script lang="ts">
    import ListQuestionValue from "../ListQuestionValue.svelte";
    import type { DecryptedSubmissionSuccess } from "../../../../../data/crypto/results";
    import type { APIQuestion } from "@paltiverse/palform-typescript-openapi";

    export let question: APIQuestion;
    export let submission: DecryptedSubmissionSuccess;

    $: questionSubmission = submission.questions.find(
        (e) => e.question_id === question.id
    );
</script>

<td class="py-2 px-3 text-sm hover:bg-slate-300 dark:hover:bg-slate-700/70">
    {#if questionSubmission}
        <ListQuestionValue {questionSubmission} {question} compact />
    {/if}
</td>
