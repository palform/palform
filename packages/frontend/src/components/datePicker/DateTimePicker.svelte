<script lang="ts">
    import { DateTime } from "luxon";
    import DatePicker from "./DatePicker.svelte";
    import TimePicker from "./TimePicker.svelte";
    import { createEventDispatcher } from "svelte";
    import InfoText from "../type/InfoText.svelte";
    import { timeZoneSummary } from "../../data/util/time";

    export let id: string | undefined = undefined;
    export let disabled = false;
    export let selectedDateTime: string | null;
    export let min: string | undefined = undefined;
    export let max: string | undefined = undefined;

    export let pickDate = true;
    export let pickTime = true;

    let parsedDateTime = selectedDateTime
        ? DateTime.fromISO(selectedDateTime)
        : null;

    const dispatch = createEventDispatcher<{ update: string }>();

    $: onChange = () => {
        if (parsedDateTime) {
            const i = parsedDateTime.toISO();
            if (!i) return;
            selectedDateTime = i;
            dispatch("update", i);
        }
    };

    $: parsedMin = min ? DateTime.fromISO(min) : undefined;
    $: parsedMax = max ? DateTime.fromISO(max) : undefined;
</script>

<div class={`flex gap-4 flex-col md:flex-row ${$$props.class}`} {id}>
    {#if pickDate}
        <DatePicker
            class="flex-1"
            bind:selectedDate={parsedDateTime}
            {disabled}
            on:update={onChange}
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
