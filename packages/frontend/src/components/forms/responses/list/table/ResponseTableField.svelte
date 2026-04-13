<script lang="ts">
    import ListQuestionValue from "../ListQuestionValue.svelte";
    import type { DecryptedSubmissionSuccess } from "../../../../../data/crypto/results";
    import type { APIQuestion } from "@palform/palform-typescript-openapi";
    import { Modal } from "flowbite-svelte";
    import FormResponseListPages from "../pages/FormResponseListPages.svelte";

    interface Props {
        question: APIQuestion;
        submission: DecryptedSubmissionSuccess;
        submissionIndex: number;
    }

    let { question, submission, submissionIndex }: Props = $props();

    let questionSubmission = $derived(submission.questions.find(
        (e) => e.question_id === question.id
    ));

    let showIndividualResponseModal = $state(false);
</script>

<td class="min-w-48 text-sm hover:bg-slate-300 dark:hover:bg-slate-700/70">
    <button
        class="h-12 overflow-hidden py-2 px-3 text-left w-full"
        onclick={() => (showIndividualResponseModal = true)}
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
