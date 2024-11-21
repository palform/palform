<script lang="ts">
    import type {
        APIWebhook,
        APIWebhookJob,
    } from "@paltiverse/palform-typescript-openapi";
    import { APIs } from "../../../../data/common";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../../data/contexts/orgLayout";
    import TableContainer from "../../../tables/TableContainer.svelte";
    import {
        Alert,
        Spinner,
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import { parseServerTime } from "../../../../data/util/time";
    import { DateTime } from "luxon";

    export let webhook: APIWebhook;
    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();
    let jobs: APIWebhookJob[] | undefined = undefined;

    APIs.webhooks()
        .then((a) =>
            a.webhooksListJobs($orgCtx.org.id, $formCtx.id, webhook.id)
        )
        .then((resp) => (jobs = resp.data));
</script>

{#if jobs === undefined}
    <Spinner />
{:else if jobs.length === 0}
    <Alert>
        This webhook hasn't run yet. When it does, you'll see a log of runs
        here.
    </Alert>
{:else}
    <TableContainer>
        <Table>
            <TableHead>
                <TableHeadCell>
                    <span class="sr-only">Endpoint</span>
                </TableHeadCell>
                <TableHeadCell>Completed</TableHeadCell>
                <TableHeadCell>Failed attempts</TableHeadCell>
                <TableHeadCell>Error</TableHeadCell>
            </TableHead>
            <TableBody>
                {#each jobs as job (job.id)}
                    <TableBodyRow>
                        <TableBodyCell class="font-mono">
                            <strong>POST</strong>
                            {webhook.endpoint}

                            {#if !job.done_at && !job.error}
                                <span class="block text-xs">Pending</span>
                            {/if}
                        </TableBodyCell>
                        <TableBodyCell>
                            {#if job.done_at}
                                {parseServerTime(job.done_at).toLocaleString(
                                    DateTime.DATETIME_MED
                                )}
                            {/if}
                        </TableBodyCell>
                        <TableBodyCell>
                            {job.retries ?? ""}
                        </TableBodyCell>
                        <TableBodyCell>
                            {job.error ?? ""}
                        </TableBodyCell>
                    </TableBodyRow>
                {/each}
            </TableBody>
        </Table>
    </TableContainer>
{/if}
