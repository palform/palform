<script lang="ts">
    import {
        Alert,
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import { getTeamCtx } from "../../../data/contexts/team";
    import TeamAddMemberModal from "../../../components/teams/TeamAddMemberModal.svelte";
    import TableContainer from "../../../components/tables/TableContainer.svelte";
    import TeamMemberRow from "../../../components/teams/TeamMemberRow.svelte";
    import type {
        APIOrganisationTeamMember,
        OrganisationMemberRoleEnum,
    } from "@palform/palform-typescript-openapi";
    import { getUserId } from "../../../data/auth";

    const teamCtx = getTeamCtx();

    let onMemberRoleChange = $derived(
        (userId: string, newRole: OrganisationMemberRoleEnum) => {
            const i = $teamCtx.members.findIndex((e) => e.user_id === userId);
            teamCtx.update((ctx) => {
                ctx.members[i].role = newRole;
                return ctx;
            });
        }
    );
    let onMemberDelete = $derived((userId: string) => {
        teamCtx.update((ctx) => {
            return {
                ...ctx,
                members: ctx.members.filter((e) => e.user_id !== userId),
            };
        });
    });
    let onMemberAdd = $derived((e: APIOrganisationTeamMember) => {
        teamCtx.update((ctx) => {
            return {
                ...ctx,
                members: [e, ...ctx.members],
            };
        });
    });

    let userId: undefined | string = $state(undefined);
    $effect(() => {
        getUserId().then((u) => {
            userId = u;
        });
    });
</script>

{#if $teamCtx.team.is_default}
    <Alert color="blue" class="mb-4" border>
        This is your default team. All users in your organisation are members of
        this team, and they cannot be removed or added.
    </Alert>
{:else}
    <TeamAddMemberModal
        class="mb-4"
        teamId={$teamCtx.team.id}
        existingTeamMemberIds={$teamCtx.members.map((e) => e.user_id)}
        onadd={onMemberAdd}
    />
{/if}

<TableContainer>
    <Table>
        <TableHead>
            <TableHeadCell>Name</TableHeadCell>
            <TableHeadCell>User ID</TableHeadCell>
            <TableHeadCell>Role</TableHeadCell>
            {#if !$teamCtx.team.is_default}
                <TableHeadCell>
                    <span class="sr-only">Actions</span>
                </TableHeadCell>
            {/if}
        </TableHead>
        <TableBody>
            {#each $teamCtx.members as member (member.user_id)}
                <TeamMemberRow
                    {member}
                    teamId={$teamCtx.team.id}
                    readonly={userId === member.user_id &&
                        ($teamCtx.team.is_default ?? false)}
                    isDefaultTeam={$teamCtx.team.is_default ?? false}
                    onupdate={(e) => onMemberRoleChange(member.user_id, e)}
                    ondelete={() => onMemberDelete(member.user_id)}
                />
            {/each}
        </TableBody>
    </Table>
</TableContainer>
