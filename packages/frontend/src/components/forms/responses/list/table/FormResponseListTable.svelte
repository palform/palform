<script lang="ts">
    import { getResponsesContext } from "../../../../../data/contexts/results";
    import ResponseTableRow from "./ResponseTableRow.svelte";

    const responsesCtx = getResponsesContext();
</script>

<div class="overflow-y-auto rounded-lg border dark:border-gray-700">
    <table class="w-full">
        <thead>
            <tr class="bg-slate-100 dark:bg-slate-700">
                {#each $responsesCtx.questions as question}
                    <th
                        class="text-left dark:text-gray-300 py-2 px-3 font-display font-medium text-sm"
                    >
                        {question.internal_name || question.title}
                    </th>
                {/each}
            </tr>
        </thead>
        <tbody>
            {#each $responsesCtx.submissions as submission}
                <ResponseTableRow {submission} />
            {/each}
        </tbody>
    </table>
</div>
