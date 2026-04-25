<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import {
        submissionIsError,
        submissionIsSuccess,
        type DecryptedSubmission,
    } from "../../../../../data/crypto/results";
    import ResponseTableField from "./ResponseTableField.svelte";
    import { getFormAdminContext } from "../../../../../data/contexts/formAdmin";

    interface Props {
        submission: DecryptedSubmission;
        submissionIndex: number;
    }

    let { submission, submissionIndex }: Props = $props();
    const formAdminCtx = getFormAdminContext();

    let observer: IntersectionObserver;
    let trElement: HTMLTableRowElement | undefined = $state();
    let inView = $state(false);

    const intersectionHandler = (
        e: IntersectionObserverEntry[],
        _: IntersectionObserver
    ) => {
        inView = e[0].isIntersecting;
    };

    onMount(() => {
        if (!trElement) return;
        observer = new IntersectionObserver(intersectionHandler, {
            root: null,
        });
        observer.observe(trElement);
    });
    onDestroy(() => {
        if (!trElement) return;
        observer.unobserve(trElement);
        observer.disconnect();
    });
</script>

{#if submissionIsError(submission)}
    <tr
        class="h-12 bg-red-50 even:bg-red-100 dark:bg-red-950 dark:even:bg-red-900"
        bind:this={trElement}
    >
        {#if inView}
            <td
                colspan={$formAdminCtx.questions.length}
                class="px-3 text-sm text-red-800 dark:text-red-200"
            >
                {submission.error}
            </td>
        {/if}
    </tr>
{/if}

{#if submissionIsSuccess(submission)}
    <tr
        class={inView
            ? "overflow-hidden even:bg-slate-50 dark:even:bg-slate-800"
            : "h-12"}
        bind:this={trElement}
    >
        {#if inView}
            {#each $formAdminCtx.questions as question (question.id)}
                <ResponseTableField {question} {submission} {submissionIndex} />
            {/each}
        {/if}
    </tr>
{/if}
