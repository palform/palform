<script lang="ts">
    import SectionSeparator from "../../../type/SectionSeparator.svelte";
    import CorrelationTarget from "./CorrelationTarget.svelte";

    interface Props {
        fromQuestionId: string;
        fromFeatureLabel: string;
        targets: [string, [string, number][]][];
    }

    let { fromQuestionId, fromFeatureLabel, targets }: Props = $props();

    let isSome = $derived(targets.some(([_, e]) => e.length > 0));
</script>

{#if isSome}
    {#if fromFeatureLabel.length > 0}
        <p>{fromFeatureLabel}</p>
    {/if}

    {#each targets as [targetQuestionId, featureStrengths] (targetQuestionId)}
        {#each featureStrengths as [targetFeatureLabel, strength] (targetFeatureLabel)}
            <CorrelationTarget
                {fromQuestionId}
                {fromFeatureLabel}
                {targetQuestionId}
                {targetFeatureLabel}
                {strength}
            />
        {/each}
    {/each}

    {#if fromFeatureLabel.length > 0}
        <SectionSeparator />
    {/if}
{/if}
