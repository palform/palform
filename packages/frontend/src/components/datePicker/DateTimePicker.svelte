<script lang="ts">
    import { DateTime } from "luxon";
    import DatePicker from "./DatePicker.svelte";
    import TimePicker from "./TimePicker.svelte";
    import { createEventDispatcher } from "svelte";
    import InfoText from "../type/InfoText.svelte";
    import { timeZoneSummary } from "../../data/util/time";

    interface Props {
        id?: string;
        disabled?: boolean;
        selectedDateTime: string | null;
        min?: string;
        max?: string;
        pickDate?: boolean;
        pickTime?: boolean;
        class?: string;
    }

    let {
        id,
        disabled = false,
        selectedDateTime = $bindable(),
        min,
        max,
        pickDate = true,
        pickTime = true,
        class: className,
    }: Props = $props();

    let parsedDateTime = $state<DateTime | null>(
        selectedDateTime ? DateTime.fromISO(selectedDateTime) : null
    );

    $effect(() => {
        parsedDateTime = selectedDateTime
            ? DateTime.fromISO(selectedDateTime)
            : null;
    });

    const dispatch = createEventDispatcher<{ update: string }>();

    function onChange() {
        if (parsedDateTime) {
            const i = parsedDateTime.toISO();
            if (!i) return;
            selectedDateTime = i;
            dispatch("update", i);
        }
    }

    let parsedMin = $derived(min ? DateTime.fromISO(min) : undefined);
    let parsedMax = $derived(max ? DateTime.fromISO(max) : undefined);
</script>

<div class={`flex gap-4 flex-col md:flex-row ${className ?? ""}`} {id}>
    {#if pickDate}
        <DatePicker
            class="flex-1"
            bind:selectedDate={parsedDateTime}
            {disabled}
            onUpdate={onChange}
            min={parsedMin}
            max={parsedMax}
        />
    {/if}
    {#if pickTime}
        <TimePicker
            class="flex-1"
            bind:selectedTime={parsedDateTime}
            {disabled}
            on:update={onChange}
            min={parsedMin}
            max={parsedMax}
        />
    {/if}
</div>

{#if parsedDateTime}
    <InfoText class="mt-2" lighter>
        {timeZoneSummary(parsedDateTime)}
    </InfoText>
{/if}
