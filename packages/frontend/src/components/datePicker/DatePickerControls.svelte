<script lang="ts">
    import {
        faCaretLeft,
        faCaretRight,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { getBrandCtx } from "../../data/contexts/brand";

    interface Props {
        currentYear: number;
        currentMonth: number;
        disabled?: boolean;
        class?: string;
        onprev: () => void;
        onnext: () => void;
    }

    let {
        currentYear,
        currentMonth,
        disabled = false,
        class: className,
        onprev,
        onnext,
    }: Props = $props();

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

    function onPrev(e: Event) {
        e.preventDefault();
        onprev();
    }
    function onNext(e: Event) {
        e.preventDefault();
        onnext();
    }
</script>

<div class={`flex justify-stretch items-center ${className ?? ""}`}>
    <button
        class="flex-1 hover:bg-gray-100 dark:hover:bg-gray-700 h-full"
        type="button"
        onclick={onPrev}
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
        onclick={onNext}
        {disabled}
    >
        <FontAwesomeIcon
            icon={faCaretRight}
            class="text-gray-500"
            color={$brandCtx?.primary_color}
        />
    </button>
</div>
