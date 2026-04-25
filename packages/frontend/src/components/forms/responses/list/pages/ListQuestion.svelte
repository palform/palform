<script lang="ts">
    import type { QuestionSubmission } from "@palform/palform-client-js-extra-types/QuestionSubmission";
    import CardBox from "../../../../cardBox/CardBox.svelte";
    import CardBoxTitle from "../../../../cardBox/CardBoxTitle.svelte";
    import ListQuestionValue from "../ListQuestionValue.svelte";
    import { getFormAdminContext } from "../../../../../data/contexts/formAdmin";

    interface Props {
        questionSubmission: QuestionSubmission;
    }

    let { questionSubmission }: Props = $props();
    const formAdminCtx = getFormAdminContext();

    let question = $derived($formAdminCtx.questions.find(
        (e) => e.id === questionSubmission.question_id
    ));
</script>

<CardBox>
    {#if question === undefined}
        <p class="text-red-600">Question not found!</p>
        <p class="text-sm">
            This question doesn't currently exist. It might have been deleted
            since this response was made.
        </p>
    {:else}
        <CardBoxTitle>
            {question.title}
        </CardBoxTitle>

        <ListQuestionValue {question} {questionSubmission} compact={false} />
    {/if}
</CardBox>
