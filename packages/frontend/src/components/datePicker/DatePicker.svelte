<script lang="ts">
    import { DateTime } from "luxon";
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../data/contexts/brand";
    import DatePickerWeekday from "./DatePickerWeekday.svelte";
    import DatePickerControls from "./DatePickerControls.svelte";
    import DatePickerDay from "./DatePickerDay.svelte";

    interface Props {
        selectedDate?: DateTime | null;
        disabled?: boolean;
        min?: DateTime | undefined;
        max?: DateTime | undefined;
        class?: string;
        onUpdate?: (date: DateTime) => void;
    }

    let {
        selectedDate = $bindable(null),
        disabled = false,
        min = undefined,
        max = undefined,
        class: className,
        onUpdate,
    }: Props = $props();

    let currentMonth = $state(selectedDate?.month ?? DateTime.now().month);
    let currentYear = $state(selectedDate?.year ?? DateTime.now().year);

    const brandCtx = getBrandCtx();
    let firstDay = $derived(
        DateTime.fromObject({
            day: 1,
            month: currentMonth,
            year: currentYear,
        }).startOf("week")
    );

    const allDays = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
        38, 39, 40, 41,
    ];

    function prevMonth() {
        currentMonth -= 1;
        if (currentMonth < 1) {
            currentYear -= 1;
            currentMonth = 12;
        }
    }
    function nextMonth() {
        currentMonth += 1;
        if (currentMonth > 12) {
            currentYear += 1;
            currentMonth = 1;
        }
    }
    function onSelect(date: DateTime) {
        currentMonth = date.month;
        currentYear = date.year;
        selectedDate = date;
        onUpdate?.(date);
    }
</script>

<div
    class={`grid grid-rows-6 grid-cols-7 border border-gray-200 dark:border-gray-600 overflow-hidden ${className ?? ""}`}
    style:border-radius={getRoundingAmountForBrand($brandCtx)}
>
    <DatePickerControls
        class="col-span-7"
        {currentMonth}
        {currentYear}
        onprev={prevMonth}
        onnext={nextMonth}
        {disabled}
    />
    <DatePickerWeekday>Mo</DatePickerWeekday>
    <DatePickerWeekday>Tu</DatePickerWeekday>
    <DatePickerWeekday>We</DatePickerWeekday>
    <DatePickerWeekday>Th</DatePickerWeekday>
    <DatePickerWeekday>Fr</DatePickerWeekday>
    <DatePickerWeekday>Sa</DatePickerWeekday>
    <DatePickerWeekday>Su</DatePickerWeekday>

    {#each allDays as dayIndex}
        <DatePickerDay
            {firstDay}
            {selectedDate}
            {dayIndex}
            month={currentMonth}
            onclick={onSelect}
            {disabled}
            {min}
            {max}
        />
    {/each}
</div>
