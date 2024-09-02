<script lang="ts">
    import { DateTime } from "luxon";
    import {
        getBrandCtx,
        getRoundingAmountForBrand,
    } from "../../data/contexts/brand";
    import DatePickerWeekday from "./DatePickerWeekday.svelte";
    import DatePickerControls from "./DatePickerControls.svelte";
    import DatePickerDay from "./DatePickerDay.svelte";
    import { createEventDispatcher } from "svelte";

    export let selectedDate: DateTime | null = null;
    export let disabled = false;
    export let min: DateTime | undefined = undefined;
    export let max: DateTime | undefined = undefined;

    const dispatch = createEventDispatcher<{ update: undefined }>();

    let currentMonth = selectedDate?.month ?? DateTime.now().month;
    let currentYear = selectedDate?.year ?? DateTime.now().year;

    const brandCtx = getBrandCtx();
    $: firstDay = DateTime.fromObject({
        day: 1,
        month: currentMonth,
        year: currentYear,
    }).startOf("week");

    $: allDays = [
        0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19,
        20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37,
        38, 39, 40, 41,
    ];

    $: prevMonth = () => {
        currentMonth -= 1;
        if (currentMonth < 1) {
            currentYear -= 1;
            currentMonth = 12;
        }
    };
    $: nextMonth = () => {
        currentMonth += 1;
        if (currentMonth > 12) {
            currentYear += 1;
            currentMonth = 1;
        }
    };
    $: onSelect = (e: CustomEvent<DateTime>) => {
        currentMonth = e.detail.month;
        currentYear = e.detail.year;
        selectedDate = e.detail;
        dispatch("update");
    };
</script>

<div
    class={`grid grid-rows-6 grid-cols-7 border dark:border-gray-600 overflow-hidden ${$$props.class}`}
    style:border-radius={getRoundingAmountForBrand($brandCtx)}
>
    <DatePickerControls
        class="col-span-7"
        {currentMonth}
        {currentYear}
        on:prev={prevMonth}
        on:next={nextMonth}
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
            on:click={onSelect}
            {disabled}
            {min}
            {max}
        />
    {/each}
</div>
