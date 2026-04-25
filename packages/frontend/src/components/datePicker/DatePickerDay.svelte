<script lang="ts">
    import { run } from "svelte/legacy";

    import type { DateTime } from "luxon";
    import { isDateOnlyEqual } from "../../data/util/time";
    import { getBrandCtx } from "../../data/contexts/brand";

    interface Props {
        firstDay: DateTime;
        selectedDate: DateTime | null;
        dayIndex: number;
        month: number;
        disabled?: boolean;
        min?: DateTime | undefined;
        max?: DateTime | undefined;
        onclick?: (date: DateTime) => void;
    }

    let {
        firstDay,
        selectedDate,
        dayIndex,
        month,
        disabled = false,
        min = undefined,
        max = undefined,
        onclick,
    }: Props = $props();

    const brandCtx = getBrandCtx();

    let thisDay = $derived(firstDay.plus({ days: dayIndex }));
    let selected = $derived(
        selectedDate ? isDateOnlyEqual(selectedDate, thisDay) : false
    );

    let minMaxDisabled = $state(false);
    run(() => {
        minMaxDisabled = false;
        if (min && thisDay < min.startOf("day")) {
            minMaxDisabled = true;
        }
        if (max && thisDay > max.endOf("day")) {
            minMaxDisabled = true;
        }
    });

    let anyDisabled = $derived(minMaxDisabled || disabled);

    let onClick = $derived((e: Event) => {
        e.preventDefault();
        onclick?.(thisDay);
    });
</script>

<button
    class={`${selected ? "bg-primary-600 text-white" : "hover:bg-gray-100 dark:hover:bg-gray-700 text:gray-700 dark:text-gray-300"} ${anyDisabled ? "text-gray-300 dark:text-gray-700" : thisDay.month !== month ? "text-gray-500 dark:text-gray-500" : ""} py-1`}
    onclick={onClick}
    type="button"
    disabled={anyDisabled}
    style:background-color={selected ? $brandCtx?.primary_color : undefined}
>
    {thisDay.day}
</button>
