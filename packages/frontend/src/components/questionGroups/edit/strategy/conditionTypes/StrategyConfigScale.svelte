<script lang="ts">
    import {
        DirectionOperator,
        type APIQuestionConfigurationOneOf3Scale,
        type APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
    } from "@paltiverse/palform-typescript-openapi";
    import { Button, Label, Select } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import { genScaleList } from "../../../../../data/util/scaleList";
    import { comparisonItems } from "../../../../../data/util/directionOperator";

    export let configuration: APIQuestionConfigurationOneOf3Scale;
    const dispatch = createEventDispatcher<{
        save: APIQuestionGroupStepStrategyJumpCaseConditionMatcher;
    }>();

    let operator = "";

    $: comparableItems = genScaleList(
        operator === "GreaterThan" || operator === "Equal"
            ? configuration.min
            : configuration.min + 1,
        operator === "LessThan" || operator === "Equal"
            ? configuration.max
            : configuration.max - 1
    ).map((e) => ({ name: e.toString(), value: e }));
    let comparedValue = 0;

    $: onSave = () => {
        if (
            comparedValue < configuration.min ||
            comparedValue > configuration.max
        )
            return;

        dispatch("save", {
            Scale: {
                direction: operator as DirectionOperator,
                value: comparedValue,
            },
        });
    };
</script>

<Label>
    Selected value is
    <Select class="mt-2" items={comparisonItems} bind:value={operator} />
</Label>

{#if operator !== ""}
    <Select class="mt-4" items={comparableItems} bind:value={comparedValue} />
    <Button class="mt-4" size="sm" on:click={onSave}>Save</Button>
{/if}
