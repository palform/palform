<script lang="ts">
    import ListQuestionValue from "../ListQuestionValue.svelte";
    import type { DecryptedSubmissionSuccess } from "../../../../../data/crypto/results";
    import type { APIQuestion } from "@paltiverse/palform-typescript-openapi";
    import { Modal } from "flowbite-svelte";
    import FormResponseListPages from "../pages/FormResponseListPages.svelte";

    export let question: APIQuestion;
    export let submission: DecryptedSubmissionSuccess;
    export let submissionIndex: number;

    $: questionSubmission = submission.questions.find(
        (e) => e.question_id === question.id
    );

    let showIndividualResponseModal = false;
</script>

<td class="min-w-48 text-sm hover:bg-slate-300 dark:hover:bg-slate-700/70">
    <button
        class="h-12 overflow-hidden py-2 px-3 text-left w-full"
        on:click={() => (showIndividualResponseModal = true)}
    >
        {#if questionSubmission}
            <ListQuestionValue {questionSubmission} {question} compact />
        {/if}
    </button>
</td>

<Modal
    title="Full response"
    size="xl"
    outsideclose
    bind:open={showIndividualResponseModal}
>
    <div class="text-gray-800 dark:text-gray-400">
        <FormResponseListPages selectedSubmissionIndex={submissionIndex} />
    </div>
</Modal>
