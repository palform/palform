<script lang="ts">
    import { Alert } from "flowbite-svelte";
    import { getResponsesContext } from "../../../../data/contexts/results";
    import FormResponseListTable from "./table/FormResponseListTable.svelte";

    const respCtx = getResponsesContext();
</script>

{#if $respCtx.submissions.length === 0}
    <Alert color="blue">No submissions yet.</Alert>
{:else}
    <FormResponseListTable />
{/if}
