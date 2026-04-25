<script lang="ts">
    import type {
        APIOrganisationTeamMember,
        OrganisationMemberRoleEnum,
    } from "@palform/palform-typescript-openapi";
    import {
        Select,
        TableBodyCell,
        TableBodyRow,
        Tooltip,
    } from "flowbite-svelte";
    import TableSingleAction from "../tables/TableSingleAction.svelte";
    import { orgMemberSelectItems } from "../../data/util/orgMemberEnum";
    import { APIs } from "../../data/common";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../data/toast";
    import { createEventDispatcher } from "svelte";

    interface Props {
        member: APIOrganisationTeamMember;
        teamId: string;
        isDefaultTeam: boolean;
        readonly: boolean;
    }

    let { member, teamId, isDefaultTeam, readonly }: Props = $props();
    const orgCtx = getOrgContext();

    const dispatch = createEventDispatcher<{
        update: OrganisationMemberRoleEnum;
        delete: undefined;
    }>();
    let roleValue = $state(member.role);
    let loading = $state(false);
    let onRoleChange = $derived(async () => {
        loading = true;
        try {
            await APIs.orgTeamMembers().then((a) =>
                a.organisationTeamMembersPatch(
                    $orgCtx.org.id,
                    teamId,
                    member.user_id,
                    {
                        new_role: roleValue,
                    }
                )
            );
            await showSuccessToast("Saved role");
            dispatch("update", roleValue);
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    });

    let onDelete = $derived(async () => {
        loading = true;
        try {
            await APIs.orgTeamMembers().then((a) =>
                a.organisationTeamMembersDelete(
                    $orgCtx.org.id,
                    teamId,
                    member.user_id
                )
            );
            await showSuccessToast("Removed user from team");
            dispatch("delete");
        } catch (e) {
            await showFailureToast(e);
        }
    });
</script>

<TableBodyRow>
    <TableBodyCell>
        {#if member.user_display_name}
            <span class="block font-medium">
                {member.user_display_name}
            </span>
        {/if}
        {member.user_email}
    </TableBodyCell>
    <TableBodyCell>
        {member.user_id}
    </TableBodyCell>
    <TableBodyCell>
        <Select
            items={orgMemberSelectItems()}
            bind:value={roleValue}
            onchange={onRoleChange}
            disabled={readonly}
        />
        {#if readonly}
            <Tooltip>Cannot change own role in default team</Tooltip>
        {/if}
    </TableBodyCell>
    {#if !isDefaultTeam}
        <TableBodyCell>
            <TableSingleAction onclick={onDelete}>Delete</TableSingleAction>
        </TableBodyCell>
    {/if}
</TableBodyRow>
