<script lang="ts">
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/results";
    import {
        qIsAddress,
        qIsChoice,
        qIsChoiceMatrix,
        qIsDateTime,
        qIsHidden,
        qIsPhoneNumber,
        qIsScale,
        qIsText,
    } from "../../../../data/contexts/questionsEditing";
    import CardBox from "../../../cardBox/CardBox.svelte";
    import CardBoxTitle from "../../../cardBox/CardBoxTitle.svelte";
    import OverviewChartChoice from "./OverviewChartChoice.svelte";
    import OverviewChartText from "./OverviewChartText.svelte";
    import OverviewChartScale from "./OverviewChartScale.svelte";
    import OverviewChartAddress from "./OverviewChartAddress.svelte";
    import OverviewChartPhoneNumber from "./OverviewChartPhoneNumber.svelte";
    import { getCorrelationsForQuestion } from "../../../../data/contexts/analysis/correlation";
    import CorrelationViewer from "../../analysis/correlation/CorrelationViewer.svelte";
    import { Alert } from "flowbite-svelte";
    import OverviewChartChoiceMatrix from "./OverviewChartChoiceMatrix.svelte";
    import OverviewChartDateTime from "./OverviewChartDateTime.svelte";
    import OverviewChartHidden from "./OverviewChartHidden.svelte";

    export let questionId: string;

    $: question = ctxGetQuestion(questionId);
    $: submissions = ctxSubmissionsForQuestion(questionId);
    $: correlations = getCorrelationsForQuestion(questionId);
</script>

{#if $question !== undefined}
    <CardBox>
        <CardBoxTitle>
            {$question.title}
        </CardBoxTitle>
        <p class="text-sm text-gray-500">
            {$submissions.length} responses
        </p>
        {#if $correlations !== undefined}
            <CorrelationViewer
                class="mt-1"
                correlations={$correlations}
                {questionId}
            />
        {/if}

        <div class="h-4" />
        {#if qIsText($question.configuration)}
            <OverviewChartText {questionId} />
        {:else if qIsChoice($question.configuration)}
            <OverviewChartChoice {questionId} />
        {:else if qIsScale($question.configuration)}
            <OverviewChartScale {questionId} />
        {:else if qIsAddress($question.configuration)}
            <OverviewChartAddress {questionId} />
        {:else if qIsPhoneNumber($question.configuration)}
            <OverviewChartPhoneNumber {questionId} />
        {:else if qIsChoiceMatrix($question.configuration)}
            <OverviewChartChoiceMatrix {questionId} />
        {:else if qIsDateTime($question.configuration)}
            <OverviewChartDateTime {questionId} />
        {:else if qIsHidden($question.configuration)}
            <OverviewChartHidden {questionId} />
        {:else}
            <Alert>No overview available for this type of question</Alert>
        {/if}
    </CardBox>
{/if}
