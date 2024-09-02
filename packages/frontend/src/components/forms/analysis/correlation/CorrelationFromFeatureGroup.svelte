<script lang="ts">
    import SectionSeparator from "../../../type/SectionSeparator.svelte";
    import CorrelationTarget from "./CorrelationTarget.svelte";

    export let fromQuestionId: string;
    export let fromFeatureLabel: string;
    export let targets: [string, [string, number][]][];

    $: isSome = targets.some(([_, e]) => e.length > 0);
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
