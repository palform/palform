<script lang="ts">
    import { Modal } from "flowbite-svelte";
    import { getFormAdminContext } from "../../../../../data/contexts/formAdmin";
    import ResponseTableRow from "./ResponseTableRow.svelte";
    import FormResponseListPages from "../pages/FormResponseListPages.svelte";

    const formAdminCtx = getFormAdminContext();
    let showIndividualResponseModal = $state(false);
    let modalSubmissionIndex = $state(-1);

    const openIndividualSubmissionModal = (index: number) => {
        modalSubmissionIndex = index;
        showIndividualResponseModal = true;
    };
</script>

<Modal
    title="Full response"
    size="xl"
    outsideclose
    bind:open={showIndividualResponseModal}
>
    <div class="text-gray-800 dark:text-gray-400">
        <FormResponseListPages
            bind:selectedSubmissionIndex={modalSubmissionIndex}
        />
    </div>
</Modal>

<div
    class="overflow-y-auto rounded-lg border border-gray-300 dark:border-gray-700"
>
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
                <ResponseTableRow
                    {submission}
                    openIndividualSubmissionModal={() =>
                        openIndividualSubmissionModal(index)}
                />
            {/each}
        </tbody>
    </table>
</div>
