<script lang="ts">
    import { type APIOrgMemberServiceAccount } from "@palform/palform-typescript-openapi";
    import { DropdownItem, TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import TableActions from "../../tables/TableActions.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import {
        showFailureToast,
        showSuccessToast,
        showToast,
    } from "../../../data/toast";
    import {
        faCheck,
        faCheckCircle,
        faTimesCircle,
    } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { parseServerTime } from "../../../data/util/time";
    import { DateTime } from "luxon";

    interface Props {
        member: APIOrgMemberServiceAccount;
        isSelf?: boolean;
        ondelete: () => void;
        onupdate: (member: APIOrgMemberServiceAccount) => void;
    }

    let { member, isSelf = false, ondelete, onupdate }: Props = $props();
    const orgCtx = getOrgContext();

    let loading = $state(false);
    let onDelete = $derived(async () => {
        if (loading) return;
        loading = true;
        await APIs.adminUsers().then((a) =>
            a.adminUsersServiceAccountsDelete(member.user_id, $orgCtx.org.id)
        );
        await showToast({
            label: "Service account deleted!",
            color: "green",
            icon: faCheck,
        });
        ondelete();
        loading = false;
    });

    let setAdmin = $derived(async (admin: boolean) => {
        loading = true;

        try {
            await APIs.orgMembers().then((a) =>
                a.organisationMembersPatch($orgCtx.org.id, member.user_id, {
                    is_admin: admin,
                })
            );
            onupdate({
                ...member,
                is_admin: admin,
            });
            await showSuccessToast("Service account role updated");
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    });
</script>

<TableBodyRow>
    <TableBodyCell>
        {member.user_email}
    </TableBodyCell>
    <TableBodyCell>
        {#if member.is_admin}
            <FontAwesomeIcon
                icon={faCheckCircle}
                class={"text-green-400"}
                size="xl"
            />
        {:else}
            <FontAwesomeIcon
                icon={faTimesCircle}
                class={"text-slate-600"}
                size="xl"
            />
        {/if}
    </TableBodyCell>
    <TableBodyCell
        title={parseServerTime(member.user_created_at).toLocaleString(
            DateTime.DATETIME_MED
        )}
    >
        {parseServerTime(member.user_created_at).toRelative()}
    </TableBodyCell>
    <TableBodyCell>
        <TableActions>
            <DropdownItem onclick={onDelete} disabled={isSelf}>
                Delete
            </DropdownItem>
            <DropdownItem
                onclick={() => setAdmin(!member.is_admin)}
                disabled={isSelf}
            >
                {member.is_admin ? "Remove admin powers" : "Make admin"}
            </DropdownItem>
        </TableActions>
    </TableBodyCell>
</TableBodyRow>
