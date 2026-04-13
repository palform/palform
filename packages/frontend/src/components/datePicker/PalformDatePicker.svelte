<script lang="ts">
    import { Datepicker, type DateOrRange } from "flowbite-svelte";
    import { DateTime } from "luxon";

    interface Props {
        value?: DateTime | null;
        min?: DateTime | null;
        max?: DateTime | null;
        disabled?: boolean;
        onchange: (newVal: DateTime | null) => void;
    }

    let {
        value = $bindable(null),
        min = null,
        max = null,
        disabled = false,
        onchange,
    }: Props = $props();

    let localValue = $derived(value ? value.toJSDate() : undefined);
    let onChange = $derived((date: DateOrRange) => {
        if (!(date instanceof Date)) return;
        value = DateTime.fromJSDate(date);
        onchange(value);
    });
</script>

<Datepicker
    bind:value={localValue}
    availableFrom={min?.toJSDate()}
    availableTo={max?.toJSDate()}
    onselect={onChange}
    firstDayOfWeek={1}
    {disabled}
/>
