<script lang="ts">
    import type {
        ConfigDateTimeDateTime,
        DirectionOperator,
    } from "@palform/palform-typescript-openapi";
    import { Button, Label, Select } from "flowbite-svelte";
    import { comparisonItems } from "../../../../../data/util/directionOperator";
    import DateTimePicker from "../../../../datePicker/DateTimePicker.svelte";
    import { DateTime } from "luxon";
    import type { StrategyMatcherEventProps } from "../../../../../data/contexts/formEditor";

    interface Props extends StrategyMatcherEventProps {
        configuration: ConfigDateTimeDateTime;
    }

    let { configuration, onsave }: Props = $props();

    let direction = $state("");
    let dateTime = $state(DateTime.now().toISO());
    let onSave = $derived(() => {
        if (direction === "") return;
        onsave({
            DateTime: {
                direction: direction as DirectionOperator,
                value: dateTime,
                match_date: configuration.collect_date,
                match_time: configuration.collect_time,
            },
        });
    });
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

<Button class="mt-4" size="sm" onclick={onSave}>Save</Button>
