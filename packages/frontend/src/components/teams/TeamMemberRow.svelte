<script lang="ts">
    import type {
        APIOrganisationTeamMember,
        OrganisationMemberRoleEnum,
    } from "@paltiverse/palform-typescript-openapi";
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

    export let member: APIOrganisationTeamMember;
    export let teamId: string;
    export let isDefaultTeam: boolean;
    export let readonly: boolean;
    const orgCtx = getOrgContext();

    const dispatch = createEventDispatcher<{
        update: OrganisationMemberRoleEnum;
        delete: undefined;
    }>();
    let roleValue = member.role;
    let loading = false;
    $: onRoleChange = async () => {
        loading = true;
        try {
            await APIs.orgTeamMembers().then((a) =>
                a.organisationTeamMembersPatch(
                    $orgCtx.org.id,
                    teamId,
                    member.user_id,
                    roleValue,
                ),
            );
            await showSuccessToast("Saved role");
            dispatch("update", roleValue);
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };

    $: onDelete = async () => {
        loading = true;
        try {
            await APIs.orgTeamMembers().then((a) =>
                a.organisationTeamMembersDelete(
                    $orgCtx.org.id,
                    teamId,
                    member.user_id,
                ),
            );
            await showSuccessToast("Removed user from team");
            dispatch("delete");
        } catch (e) {
            await showFailureToast(e);
        }
    };
</script>

<TableBodyRow>
    <TableBodyCell>
        {member.user_display_name}
    </TableBodyCell>
    <TableBodyCell>
        {member.user_id}
    </TableBodyCell>
    <TableBodyCell>
        <Select
            items={orgMemberSelectItems()}
            bind:value={roleValue}
            on:change={onRoleChange}
            disabled={readonly}
        />
        {#if readonly}
            <Tooltip>Cannot change own role in default team</Tooltip>
        {/if}
    </TableBodyCell>
    {#if !isDefaultTeam}
        <TableBodyCell>
            <TableSingleAction on:click={onDelete}>Delete</TableSingleAction>
        </TableBodyCell>
    {/if}
</TableBodyRow>
