<script lang="ts">
    import { slide } from "svelte/transition";
    import OverviewChart from "./OverviewChart.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faChevronDown,
        faChevronRight,
    } from "@fortawesome/free-solid-svg-icons";

    export let groupTitle: string;
    export let questions: string[];

    let expanded = true;
</script>

<button on:click={() => (expanded = !expanded)}>
    <h2 class="text-lg mb-2 text-slate-800 dark:text-slate-300">
        <span class="text-slate-400 inline-block w-4">
            {#if expanded}
                <FontAwesomeIcon icon={faChevronDown} />
            {:else}
                <FontAwesomeIcon icon={faChevronRight} />
            {/if}
        </span>
        {groupTitle}
    </h2>
</button>

{#if expanded}
    <ol
        class="space-y-4 border-l-primary-200 dark:border-l-primary-800 border-l pl-4"
        transition:slide
    >
        {#each questions as questionId (questionId)}
            <li>
                <OverviewChart {questionId} />
            </li>
        {/each}
    </ol>
{/if}
