<script lang="ts">
    import { sGetText } from "../../../../data/contexts/fill";
    import {
        ctxGetQuestion,
        ctxSubmissionsForQuestion,
    } from "../../../../data/contexts/formAdmin";
    import { qIsText } from "../../../../data/contexts/formEditor";

    export let questionId: string;
    $: question = ctxGetQuestion(questionId);
    $: submissions = ctxSubmissionsForQuestion(questionId);

    $: validSubmissions = $submissions.filter(
        (e) => sGetText(e.data).value.length > 0
    );
    $: submissionsToShow = validSubmissions.slice(0, 100);
</script>

{#if $question !== undefined && qIsText($question.configuration)}
    <div
        class={`grid ${$question.configuration.text.is_long ? "grid-cols-1" : "grid-cols-3"} rounded-lg overflow-hidden border-t border-r dark:border-slate-600 shadow-sm bg-white dark:bg-transparent`}
    >
        {#each submissionsToShow as submission}
            <p
                class="even:bg-gray-50 odd:bg-gray-100 dark:even:bg-slate-700 dark:odd:bg-slate-800 dark:text-gray-300 px-4 py-2 border-l border-b dark:border-slate-600"
            >
                {sGetText(submission.data).value}
            </p>
        {/each}
    </div>

    {#if validSubmissions.length > 100}
        <p class="mt-2 text-sm text-gray-600 dark:text-gray-400">
            Truncated to 100 submissions
        </p>
    {/if}
{/if}
