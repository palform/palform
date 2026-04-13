<script lang="ts">
    import {
        DirectionOperator,
        type ConfigScaleScale,
    } from "@palform/palform-typescript-openapi";
    import { Button, Label, Select } from "flowbite-svelte";
    import { genScaleList } from "../../../../../data/util/scaleList";
    import { comparisonItems } from "../../../../../data/util/directionOperator";
    import type { StrategyMatcherEventProps } from "../../../../../data/contexts/formEditor";

    interface Props extends StrategyMatcherEventProps {
        configuration: ConfigScaleScale;
    }

    let { configuration, onsave }: Props = $props();

    let operator = $state("");

    let comparableItems = $derived(
        genScaleList(
            operator === "GreaterThan" || operator === "Equal"
                ? configuration.min
                : configuration.min + 1,
            operator === "LessThan" || operator === "Equal"
                ? configuration.max
                : configuration.max - 1
        ).map((e) => ({ name: e.toString(), value: e }))
    );
    let comparedValue = $state(0);

    let onSave = $derived(() => {
        if (
            comparedValue < configuration.min ||
            comparedValue > configuration.max
        )
            return;

        onsave({
            Scale: {
                direction: operator as DirectionOperator,
                value: comparedValue,
            },
        });
    });
</script>

<Label>
    Selected value is
    <Select class="mt-2" items={comparisonItems} bind:value={operator} />
</Label>

{#if operator !== ""}
    <Select class="mt-4" items={comparableItems} bind:value={comparedValue} />
    <Button class="mt-4" size="sm" onclick={onSave}>Save</Button>
{/if}
