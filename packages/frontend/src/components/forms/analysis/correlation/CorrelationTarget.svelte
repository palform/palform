<script lang="ts">
    import { Alert } from "flowbite-svelte";
    import InfoText from "../../../type/InfoText.svelte";
    import FormQuestionPathLabel from "../../FormQuestionPathLabel.svelte";
    import TextButton from "../../../TextButton.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faChevronDown,
        faChevronUp,
    } from "@fortawesome/free-solid-svg-icons";
    import CorrelationGraph from "./CorrelationGraph.svelte";

    interface Props {
        fromQuestionId: string;
        fromFeatureLabel: string;
        targetQuestionId: string;
        targetFeatureLabel: string;
        strength: number;
    }

    let {
        fromQuestionId,
        fromFeatureLabel,
        targetQuestionId,
        targetFeatureLabel,
        strength,
    }: Props = $props();

    let showGraph = $state(false);
</script>

<Alert color="gray">
    <InfoText>
        <FormQuestionPathLabel
            questionId={targetQuestionId}
            featureName={targetFeatureLabel}
        />
    </InfoText>
    <InfoText lighter>
        has a <strong>{(Math.abs(strength) * 100).toFixed(2)}%</strong> correlation
    </InfoText>
    <TextButton onclick={() => (showGraph = !showGraph)}>
        {#if showGraph}
            <FontAwesomeIcon icon={faChevronUp} />
        {:else}
            <FontAwesomeIcon icon={faChevronDown} />
        {/if}
        Graph
    </TextButton>

    {#if showGraph}
        <CorrelationGraph
            fromId={fromQuestionId}
            fromFeature={fromFeatureLabel}
            toId={targetQuestionId}
            toFeature={targetFeatureLabel}
        />
    {/if}
</Alert>
