<script lang="ts">
    import { Alert } from "flowbite-svelte";
    import FormResponseListTable from "./table/FormResponseListTable.svelte";
    import { getFormAdminContext } from "../../../../data/contexts/formAdmin";

    const formAdminCtx = getFormAdminContext();
</script>

{#if $formAdminCtx.submissions.length === 0}
    <Alert color="blue">No submissions yet.</Alert>
{:else}
    <FormResponseListTable />
{/if}
