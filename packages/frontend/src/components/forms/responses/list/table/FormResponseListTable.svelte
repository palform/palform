<script lang="ts">
    import { getFormAdminContext } from "../../../../../data/contexts/formAdmin";
    import ResponseTableRow from "./ResponseTableRow.svelte";

    const formAdminCtx = getFormAdminContext();
</script>

<div class="overflow-y-auto rounded-lg border dark:border-gray-700">
    <table class="w-full">
        <thead>
            <tr class="bg-slate-100 dark:bg-slate-700">
                {#each $formAdminCtx.questions as question}
                    <th
                        class="text-left dark:text-gray-300 py-2 px-3 font-display font-medium text-sm"
                    >
                        {question.internal_name || question.title}
                    </th>
                {/each}
            </tr>
        </thead>
        <tbody>
            {#each $formAdminCtx.submissions as submission, index}
                <ResponseTableRow {submission} submissionIndex={index} />
            {/each}
        </tbody>
    </table>
</div>
