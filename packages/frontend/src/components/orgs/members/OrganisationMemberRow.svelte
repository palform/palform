<script lang="ts">
    import { type APIOrgMember } from "@paltiverse/palform-typescript-openapi";
    import { DropdownItem, TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import TableActions from "../../tables/TableActions.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { createEventDispatcher } from "svelte";
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

    export let member: APIOrgMember;
    export let isSelf = false;
    const dispatch = createEventDispatcher<{
        delete: undefined;
        update: APIOrgMember;
    }>();

    const orgCtx = getOrgContext();

    let loading = false;
    $: onDelete = async () => {
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
        dispatch("delete");
        loading = false;
    };

    $: setAdmin = async (admin: boolean) => {
        loading = true;

        try {
            await APIs.orgMembers().then((a) =>
                a.organisationMembersPatch($orgCtx.org.id, member.user_id, {
                    is_admin: admin,
                })
            );
            dispatch("update", {
                ...member,
                is_admin: admin,
            });
            await showSuccessToast("Member role updated");
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };
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
            <DropdownItem on:click={onDelete} disabled={isSelf}>
                Delete
            </DropdownItem>
            <DropdownItem
                on:click={() => setAdmin(!member.is_admin)}
                disabled={isSelf}
            >
                {member.is_admin ? "Remove admin powers" : "Make admin"}
            </DropdownItem>
        </TableActions>
    </TableBodyCell>
</TableBodyRow>
