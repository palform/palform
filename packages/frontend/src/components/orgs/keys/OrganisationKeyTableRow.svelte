<script lang="ts">
    import type { APIUserKeyWithIdentity } from "@paltiverse/palform-typescript-openapi";
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";
    import TableSingleAction from "../../tables/TableSingleAction.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { createEventDispatcher } from "svelte";

    export let key: APIUserKeyWithIdentity;
    const dispatch = createEventDispatcher<{ delete: undefined }>();
    const orgCtx = getOrgContext();
    $: createdAt = parseServerTime(key.created_at);
    $: expiresAt = parseServerTime(key.expires_at);
    $: expired = expiresAt < DateTime.now();

    let loading = false;
    $: onDelete = async () => {
        loading = true;
        try {
            await APIs.keys().then((a) => a.keysDelete($orgCtx.org.id, key.id));
            dispatch("delete");
            await showSuccessToast("Key deleted");
            loading = false;
        } catch (e) {
            await showFailureToast(e);
        }
    };
</script>

<TableBodyRow>
    <TableBodyCell>
        {key.user_display_name}
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
        <TableSingleAction on:click={onDelete}>Delete</TableSingleAction>
    </TableBodyCell>
</TableBodyRow>
