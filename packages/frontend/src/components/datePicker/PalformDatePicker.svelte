<script lang="ts">
    import { Input } from "flowbite-svelte";
    import { DateTime } from "luxon";
    import { createEventDispatcher } from "svelte";
    import SveltyPicker from "svelty-picker";

    export let value: DateTime | null = null;
    export let min: DateTime | null = null;
    export let max: DateTime | null = null;
    export let disabled = false;

    const dispatch = createEventDispatcher<{ change: DateTime | null }>();

    $: localValue = value ? value.toFormat("yyyy-MM-dd") : null;
    $: onDateChange = (e: CustomEvent<string>) => {
        if (!e.detail) {
            dispatch("change", null);
            return;
        }
        value = DateTime.fromFormat(e.detail, "yyyy-MM-dd");
        dispatch("change", value);
    };
</script>

<SveltyPicker
    bind:value={localValue}
    on:change={onDateChange}
    autocommit
    startDate={min ? min.toJSDate() : null}
    endDate={max ? max.toJSDate() : null}
    {disabled}
>
    <svelte:fragment
        slot="inputs"
        let:displayValue
        let:onInputFocus
        let:onInputBlur
        let:onKeyDown
    >
        <Input
            value={displayValue}
            on:keydown={onKeyDown}
            on:focus={onInputFocus}
            on:blur={onInputBlur}
            {disabled}
            readonly
        />
    </svelte:fragment>
</SveltyPicker>
