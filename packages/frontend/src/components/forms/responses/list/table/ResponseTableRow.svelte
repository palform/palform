<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import { getResponsesContext } from "../../../../../data/contexts/results";
    import {
        submissionIsError,
        submissionIsSuccess,
        type DecryptedSubmission,
    } from "../../../../../data/crypto/results";
    import ResponseTableField from "./ResponseTableField.svelte";

    export let submission: DecryptedSubmission;
    const respCtx = getResponsesContext();

    let observer: IntersectionObserver;
    let trElement: HTMLTableRowElement;
    let inView = false;

    const intersectionHandler = (
        e: IntersectionObserverEntry[],
        _: IntersectionObserver
    ) => {
        inView = e[0].isIntersecting;
    };

    onMount(() => {
        observer = new IntersectionObserver(intersectionHandler, {
            root: null,
        });
        observer.observe(trElement);
    });
    onDestroy(() => {
        observer.unobserve(trElement);
        observer.disconnect();
    });
</script>

{#if submissionIsError(submission)}
    <p>ERROR SUBMISSION (to do)</p>
{/if}

{#if submissionIsSuccess(submission)}
    <tr
        class={`h-12 ${inView ? "overflow-hidden even:bg-slate-50 dark:even:bg-slate-800" : ""}`}
        bind:this={trElement}
    >
        {#if inView}
            {#each $respCtx.questions as question (question.id)}
                <ResponseTableField {question} {submission} />
            {/each}
        {/if}
    </tr>
{/if}
