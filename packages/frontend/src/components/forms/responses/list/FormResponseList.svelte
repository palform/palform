<script lang="ts">
    import { Alert, TabItem, Tabs } from "flowbite-svelte";
    import { getResponsesContext } from "../../../../data/contexts/results";
    import FormResponseListPages from "./pages/FormResponseListPages.svelte";
    import FormResponseListTable from "./table/FormResponseListTable.svelte";

    const respCtx = getResponsesContext();
</script>

{#if $respCtx.submissions.length === 0}
    <Alert color="blue">No submissions yet.</Alert>
{:else}
    <Tabs contentClass="mt-4">
        <TabItem title="Table" open>
            <FormResponseListTable />
        </TabItem>
        <TabItem title="Individual">
            <FormResponseListPages />
        </TabItem>
    </Tabs>
{/if}
