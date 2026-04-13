<script lang="ts">
    import type { APIOrganisationInvite } from "@palform/palform-typescript-openapi";
    import { TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import TableSingleAction from "../../tables/TableSingleAction.svelte";
    import { copyOrgInviteLink } from "../../../data/orgInvites";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";
    import { APIs } from "../../../data/common";
    import { showToast } from "../../../data/toast";
    import { faCheck } from "@fortawesome/free-solid-svg-icons";

    interface Props {
        invite: APIOrganisationInvite;
        ondelete: () => void;
    }

    let { invite, ondelete }: Props = $props();
    const orgCtx = getOrgContext();

    let expiresAt = $derived(parseServerTime(invite.expires_at));

    let onURLClick = $derived(async () => {
        await copyOrgInviteLink($orgCtx.org.id, invite.id);
    });

    let deleteLoading = $state(false);
    let onDelete = $derived(async () => {
        deleteLoading = true;
        await APIs.orgInvites().then((a) =>
            a.organisationInvitesDelete($orgCtx.org.id, invite.id)
        );
        deleteLoading = false;
        ondelete();

        await showToast({
            label: "Invite deleted! That link can no longer be used to join your organisation.",
            color: "green",
            icon: faCheck,
        });
    });
</script>

<TableBodyRow>
    <TableBodyCell>
        <TableSingleAction onclick={onURLClick} disabled={deleteLoading}>
            {invite.id}
        </TableSingleAction>
    </TableBodyCell>
    <TableBodyCell>
        {invite.single_use ? "Yes" : "No"}
    </TableBodyCell>
    <TableBodyCell>
        {parseServerTime(invite.created_at).toLocaleString(
            DateTime.DATETIME_MED
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
        <TableSingleAction onclick={onDelete} disabled={deleteLoading}>
            Delete
        </TableSingleAction>
    </TableBodyCell>
</TableBodyRow>
