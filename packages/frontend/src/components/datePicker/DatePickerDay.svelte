<script lang="ts">
    import type { DateTime } from "luxon";
    import { createEventDispatcher } from "svelte";
    import { isDateOnlyEqual } from "../../data/util/time";
    import { getBrandCtx } from "../../data/contexts/brand";

    export let firstDay: DateTime;
    export let selectedDate: DateTime | null;
    export let dayIndex: number;
    export let month: number;
    export let disabled = false;
    export let min: DateTime | undefined = undefined;
    export let max: DateTime | undefined = undefined;

    const brandCtx = getBrandCtx();

    $: thisDay = firstDay.plus({ days: dayIndex });
    $: selected = selectedDate ? isDateOnlyEqual(selectedDate, thisDay) : false;

    let minMaxDisabled = false;
    $: {
        minMaxDisabled = false;
        if (min && thisDay < min.startOf("day")) {
            minMaxDisabled = true;
        }
        if (max && thisDay > max.endOf("day")) {
            minMaxDisabled = true;
        }
    }

    $: anyDisabled = minMaxDisabled || disabled;

    const dispatch = createEventDispatcher<{ click: DateTime }>();
    $: onClick = (e: Event) => {
        e.preventDefault();
        dispatch("click", thisDay);
    };
</script>

<button
    class={`${selected ? "bg-primary-600 text-white" : "hover:bg-gray-100 dark:hover:bg-gray-700 text:gray-700 dark:text-gray-300"} ${anyDisabled ? "text-gray-300 dark:text-gray-700" : thisDay.month !== month ? "text-gray-500 dark:text-gray-500" : ""} py-1`}
    on:click={onClick}
    type="button"
    disabled={anyDisabled}
    style:background-color={selected ? $brandCtx?.primary_color : undefined}
>
    {thisDay.day}
</button>
