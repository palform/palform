<script lang="ts">
    import type { APIAuditLogEntry } from "@palform/palform-typescript-openapi";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import SkeletonPrimitive from "../../SkeletonPrimitive.svelte";
    import TableContainer from "../../tables/TableContainer.svelte";
    import {
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import AuditLogEntryRow from "./AuditLogEntryRow.svelte";
    import AuditLogFilters from "./AuditLogFilters.svelte";
    import { DateTime } from "luxon";
    import { onMount } from "svelte";
    import type { AuditLogRequestFilters } from "../../../data/audit/filters";

    const orgCtx = getOrgContext();
    let logEntries: APIAuditLogEntry[] | undefined = $state([]);
    let logLoading = $state(true);

    let filters: AuditLogRequestFilters = $state({
        from: DateTime.now().minus({ day: 3 }).toISO(),
        to: null,
        resource: null,
    });

    let loadEntries = $derived(() => {
        logLoading = true;
        const { from, to, resource } = filters;
        APIs.audit()
            .then((a) => a.auditList($orgCtx.org.id, from, resource, to))
            .then((resp) => {
                logEntries = resp.data;
                logLoading = false;
            });
    });

    onMount(() => {
        loadEntries();
    });
</script>

<AuditLogFilters class="mb-4" bind:filters onreload={() => loadEntries()} />

{#if logLoading}
    <SkeletonPrimitive className="p-4 space-y-2">
        <SkeletonPrimitive height="40px" />
        <SkeletonPrimitive height="40px" />
        <SkeletonPrimitive height="40px" />
        <SkeletonPrimitive height="40px" />
        <SkeletonPrimitive height="40px" />
        <SkeletonPrimitive height="40px" />
    </SkeletonPrimitive>
{:else if logEntries !== undefined}
    <TableContainer>
        <Table divClass="">
            <TableHead>
                <TableHeadCell>User</TableHeadCell>
                <TableHeadCell>Action</TableHeadCell>
                <TableHeadCell>Resource</TableHeadCell>
                <TableHeadCell>Time</TableHeadCell>
                <TableHeadCell>Note</TableHeadCell>
            </TableHead>
            <TableBody>
                {#each logEntries as logEntry (logEntry.id)}
                    <AuditLogEntryRow entry={logEntry} />
                {/each}
            </TableBody>
        </Table>
    </TableContainer>
{/if}
