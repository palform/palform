<script lang="ts">
    import { type APIOrgMember } from "@palform/palform-typescript-openapi";
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

    interface Props {
        member: APIOrgMember;
        isSelf?: boolean;
        ondelete: () => void;
        onupdate: (member: APIOrgMember) => void;
    }

    let { member, isSelf = false, ondelete, onupdate }: Props = $props();
    const orgCtx = getOrgContext();

    let loading = $state(false);
    let onDelete = $derived(async () => {
        if (loading) return;
        loading = true;
        await APIs.orgMembers().then((a) =>
            a.organisationMembersDelete($orgCtx.org.id, member.user_id)
        );
        await showToast({
            label: "Member deleted!",
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
            await showSuccessToast("Member role updated");
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    });
</script>

<TableBodyRow>
    <TableBodyCell>
        {#if member.user_display_name}
            <span class="block font-medium">{member.user_display_name}</span>
        {/if}
        {member.user_email}
    </TableBodyCell>
    <TableBodyCell>
        {member.user_id}
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
