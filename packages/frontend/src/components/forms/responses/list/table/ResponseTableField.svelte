<script lang="ts">
    import ListQuestionValue from "../ListQuestionValue.svelte";
    import type { DecryptedSubmissionSuccess } from "../../../../../data/crypto/results";
    import type { APIQuestion } from "@palform/palform-typescript-openapi";

    interface Props {
        question: APIQuestion;
        submission: DecryptedSubmissionSuccess;
        openIndividualSubmissionModal: () => void;
    }

    let { question, submission, openIndividualSubmissionModal }: Props =
        $props();

    let questionSubmission = $derived(
        submission.questions.find((e) => e.question_id === question.id)
    );
</script>

<td class="min-w-48 text-sm hover:bg-slate-300 dark:hover:bg-slate-700/70">
    <button
        class="h-12 overflow-hidden py-2 px-3 text-left w-full"
        onclick={openIndividualSubmissionModal}
    >
        {#if questionSubmission}
            <ListQuestionValue {questionSubmission} {question} compact />
        {/if}
    </button>
</td>
