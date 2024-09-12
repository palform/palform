<script lang="ts">
    import type { QuestionSubmission } from "@paltiverse/palform-client-js-extra-types/QuestionSubmission";
    import CardBox from "../../../../cardBox/CardBox.svelte";
    import CardBoxTitle from "../../../../cardBox/CardBoxTitle.svelte";
    import ListQuestionValue from "../ListQuestionValue.svelte";
    import { getFormAdminContext } from "../../../../../data/contexts/formAdmin";

    export let questionSubmission: QuestionSubmission;
    const formAdminCtx = getFormAdminContext();

    $: question = $formAdminCtx.questions.find(
        (e) => e.id === questionSubmission.question_id
    );
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
