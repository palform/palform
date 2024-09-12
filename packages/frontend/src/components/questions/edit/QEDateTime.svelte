<script lang="ts">
    import type { APIQuestionConfigurationOneOf9 } from "@paltiverse/palform-typescript-openapi";
    import { createEventDispatcher } from "svelte";
    import { Alert, Button, Label, Toggle } from "flowbite-svelte";
    import { DateTime } from "luxon";
    import TextButton from "../../TextButton.svelte";
    import DateTimePicker from "../../datePicker/DateTimePicker.svelte";
    import { getFormEditorCtx, type QuestionEditEvents } from "../../../data/contexts/formEditor";

    export let config: APIQuestionConfigurationOneOf9;
    const ctx = getFormEditorCtx();
    const dispatch = createEventDispatcher<QuestionEditEvents>();

    $: onUpdate = () => {
        dispatch("update", config);
    };
</script>

{#if !config.date_time.collect_date && !config.date_time.collect_time}
    <Alert color="red" class="mb-4">
        Please choose at least one of date or time to collect
    </Alert>
{/if}

<Toggle
    bind:checked={config.date_time.collect_date}
    on:change={onUpdate}
    disabled={$ctx.loading}
>
    Collect date
</Toggle>

<Toggle
    class="mt-4"
    bind:checked={config.date_time.collect_time}
    on:change={onUpdate}
    disabled={$ctx.loading}
>
    Collect time
</Toggle>

{#if !config.date_time.min}
    <Button
        class="mt-4"
        size="sm"
        on:click={() => (config.date_time.min = DateTime.now().toISO())}
        disabled={$ctx.loading}
    >
        Add minimum date/time
    </Button>
{:else}
    <Label for="min" class="mt-4">Minimum date/time</Label>
    <DateTimePicker
        id="min"
        class="mt-2"
        bind:selectedDateTime={config.date_time.min}
        disabled={$ctx.loading}
    />

    <TextButton
        class="mt-2"
        on:click={() => (config.date_time.min = null)}
        disabled={$ctx.loading}
    >
        Delete minimum date/time
    </TextButton>
{/if}

{#if !config.date_time.max}
    <Button
        class="mt-4"
        size="sm"
        on:click={() => (config.date_time.max = DateTime.now().toISO())}
        disabled={$ctx.loading}
    >
        Add maximum date/time
    </Button>
{:else}
    <Label for="min" class="mt-4">Maximum date/time</Label>
    <DateTimePicker
        id="min"
        class="mt-2"
        bind:selectedDateTime={config.date_time.max}
        disabled={$ctx.loading}
    />

    <TextButton
        class="mt-2"
        on:click={() => (config.date_time.max = null)}
        disabled={$ctx.loading}
    >
        Delete maximum date/time
    </TextButton>
{/if}
