<script lang="ts">
    import {
        faCaretLeft,
        faCaretRight,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { getBrandCtx } from "../../data/contexts/brand";
    import { createEventDispatcher } from "svelte";

    export let currentYear: number;
    export let currentMonth: number;
    export let disabled = false;

    const brandCtx = getBrandCtx();
    const months = [
        "Jan",
        "Feb",
        "Mar",
        "Apr",
        "May",
        "Jun",
        "Jul",
        "Aug",
        "Sep",
        "Oct",
        "Nov",
        "Dec",
    ];

    const dispatch = createEventDispatcher<{
        next: undefined;
        prev: undefined;
    }>();

    $: onPrev = (e: Event) => {
        e.preventDefault();
        dispatch("prev")
    };
    $: onNext = (e: Event) => {
        e.preventDefault();
        dispatch("next")
    };
</script>

<div class={`flex justify-stretch items-center ${$$props.class}`}>
    <button
        class="flex-1 hover:bg-gray-100 dark:hover:bg-gray-700 h-full"
        type="button"
        on:click={onPrev}
        {disabled}
    >
        <FontAwesomeIcon
            icon={faCaretLeft}
            class="text-gray-500"
            color={$brandCtx?.primary_color}
        />
    </button>
    <p class="flex-1 text-center text-gray-500 dark:text-gray-400">
        {months[currentMonth - 1]}
        {currentYear}
    </p>
    <button
        class="flex-1 hover:bg-gray-100 dark:hover:bg-gray-700 h-full"
        type="button"
        on:click={onNext}
        {disabled}
    >
        <FontAwesomeIcon
            icon={faCaretRight}
            class="text-gray-500"
            color={$brandCtx?.primary_color}
        />
    </button>
</div>
