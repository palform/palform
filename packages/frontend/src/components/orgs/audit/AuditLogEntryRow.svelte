<script lang="ts">
    import type { APIAuditLogEntry } from "@palform/palform-typescript-openapi";
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import AuditLogTargetResource from "./AuditLogTargetResource.svelte";
    import AuditLogAction from "./AuditLogAction.svelte";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";

    interface Props {
        entry: APIAuditLogEntry;
    }

    let { entry }: Props = $props();
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
        <AuditLogTargetResource {entry} />
    </TableBodyCell>
    <TableBodyCell>
        {parseServerTime(entry.created_at).toLocaleString(
            DateTime.DATETIME_MED
        )}
    </TableBodyCell>
    <TableBodyCell>
        {entry.note ?? "(none)"}
    </TableBodyCell>
</TableBodyRow>
