<script lang="ts">
    import type { APIOrganisationInvite } from "@paltiverse/palform-typescript-openapi";
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import TableSingleAction from "../../tables/TableSingleAction.svelte";
    import { copyOrgInviteLink } from "../../../data/orgInvites";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";
    import { APIs } from "../../../data/common";
    import { createEventDispatcher } from "svelte";
    import { showToast } from "../../../data/toast";
    import { faCheck } from "@fortawesome/free-solid-svg-icons";

    export let invite: APIOrganisationInvite;
    const orgCtx = getOrgContext();
    const dispatch = createEventDispatcher<{ delete: undefined }>();

    $: expiresAt = parseServerTime(invite.expires_at);

    $: onURLClick = async () => {
        await copyOrgInviteLink($orgCtx.org.id, invite.id);
    };

    let deleteLoading = false;
    $: onDelete = async () => {
        deleteLoading = true;
        await APIs.orgInvites().then((a) =>
            a.organisationInvitesDelete($orgCtx.org.id, invite.id),
        );
        deleteLoading = false;
        dispatch("delete");

        await showToast({
            label: "Invite deleted! That link can no longer be used to join your organisation.",
            color: "green",
            icon: faCheck,
        });
    };
</script>

<TableBodyRow>
    <TableBodyCell>
        <TableSingleAction on:click={onURLClick} disabled={deleteLoading}>
            {invite.id}
        </TableSingleAction>
    </TableBodyCell>
    <TableBodyCell>
        {invite.single_use ? "Yes" : "No"}
    </TableBodyCell>
    <TableBodyCell>
        {parseServerTime(invite.created_at).toLocaleString(
            DateTime.DATETIME_MED,
        )}
    </TableBodyCell>
    <TableBodyCell>
        {#if expiresAt < DateTime.now()}
            <strong>Expired</strong> {expiresAt.toRelative()}
        {:else}
            {expiresAt.toLocaleString(DateTime.DATETIME_MED)} ({expiresAt.toRelative()})
        {/if}
    </TableBodyCell>
    <TableBodyCell>
        <TableSingleAction on:click={onDelete} disabled={deleteLoading}>
            Delete
        </TableSingleAction>
    </TableBodyCell>
</TableBodyRow>
