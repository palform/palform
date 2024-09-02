<script lang="ts">
    import type { APIAuditLogEntry } from "@paltiverse/palform-typescript-openapi";
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import AuditLogTargetResource from "./AuditLogTargetResource.svelte";
    import AuditLogAction from "./AuditLogAction.svelte";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";

    export let entry: APIAuditLogEntry;
</script>

<TableBodyRow>
    <TableBodyCell>
        {entry.user_display_name}
        <span class="block text-xs font-mono">
            {entry.user_id}
        </span>
    </TableBodyCell>
    <TableBodyCell>
        <AuditLogAction action={entry.verb} />
    </TableBodyCell>
    <TableBodyCell>
        <AuditLogTargetResource
            id={entry.target_resource_id}
            resourceType={entry.target_resource_type}
        />
    </TableBodyCell>
    <TableBodyCell>
        {parseServerTime(entry.created_at).toLocaleString(
            DateTime.DATETIME_MED,
        )}
    </TableBodyCell>
    <TableBodyCell>
        {entry.note ?? "(none)"}
    </TableBodyCell>
</TableBodyRow>
