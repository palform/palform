<script lang="ts">
    import type { APIUserKeyWithIdentity } from "@palform/palform-typescript-openapi";
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";
    import TableSingleAction from "../../tables/TableSingleAction.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";

    interface Props {
        key: APIUserKeyWithIdentity;
        ondelete: () => void;
    }

    let { key, ondelete }: Props = $props();
    const orgCtx = getOrgContext();
    let createdAt = $derived(parseServerTime(key.created_at));
    let expiresAt = $derived(parseServerTime(key.expires_at));
    let expired = $derived(expiresAt < DateTime.now());

    let loading = $state(false);
    let onDelete = $derived(async () => {
        loading = true;
        try {
            await APIs.keys().then((a) => a.keysDelete($orgCtx.org.id, key.id));
            ondelete();
            await showSuccessToast("Key deleted");
            loading = false;
        } catch (e) {
            await showFailureToast(e);
        }
    });
</script>

<TableBodyRow>
    <TableBodyCell>
        {#if key.user_display_name}
            <span class="block">{key.user_display_name}</span>
        {/if}
        {key.user_email}
        <span class="block text-xs font-mono">
            {key.user_id}
        </span>
    </TableBodyCell>
    <TableBodyCell class="font-mono">
        {key.key_fingerprint}
    </TableBodyCell>
    <TableBodyCell title={createdAt.toLocaleString(DateTime.DATETIME_MED)}>
        {createdAt.toRelative()}
    </TableBodyCell>
    <TableBodyCell title={expiresAt.toLocaleString(DateTime.DATETIME_MED)}>
        <span class={expired ? "font-bold" : ""}>
            {#if expired}
                Expired
            {/if}
            {#if expiresAt.diffNow("years").years > 80}
                Never
            {:else}
                {expiresAt.toRelative()}
            {/if}
        </span>
    </TableBodyCell>
    <TableBodyCell>
        <TableSingleAction onclick={onDelete} disabled={loading}>
            Delete
        </TableSingleAction>
    </TableBodyCell>
</TableBodyRow>
