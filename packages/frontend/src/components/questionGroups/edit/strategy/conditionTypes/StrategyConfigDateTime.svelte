<script lang="ts">
    import type {
        APIQuestionConfigurationOneOf9DateTime,
        APIQuestionGroupStepStrategyJumpCaseConditionMatcher,
        DirectionOperator,
    } from "@paltiverse/palform-typescript-openapi";
    import { Button, Label, Select } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import { comparisonItems } from "../../../../../data/util/directionOperator";
    import DateTimePicker from "../../../../datePicker/DateTimePicker.svelte";
    import { DateTime } from "luxon";

    export let configuration: APIQuestionConfigurationOneOf9DateTime;
    const dispatch = createEventDispatcher<{
        save: APIQuestionGroupStepStrategyJumpCaseConditionMatcher;
    }>();

    let direction = "";
    let dateTime = DateTime.now().toISO();
    $: onSave = () => {
        if (direction === "") return;
        dispatch("save", {
            DateTime: {
                direction: direction as DirectionOperator,
                value: dateTime,
                match_date: configuration.collect_date,
                match_time: configuration.collect_time,
            },
        });
    };
</script>

<Label>
    Selected date/time is
    <Select class="mt-2" items={comparisonItems} bind:value={direction} />
</Label>

{#if direction !== ""}
    <DateTimePicker
        bind:selectedDateTime={dateTime}
        class="mt-4"
        pickDate={configuration.collect_date}
        pickTime={configuration.collect_time}
    />
{/if}

<Button class="mt-4" size="sm" on:click={onSave}>Save</Button>
