<script lang="ts">
    import type { APIOrgMember } from "@paltiverse/palform-typescript-openapi";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { APIs } from "../../data/common";
    import {
        Button,
        Spinner,
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
        Tooltip,
    } from "flowbite-svelte";
    import OrganisationMemberRow from "../../components/orgs/members/OrganisationMemberRow.svelte";
    import { navigateEvent } from "../../utils/navigate";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faShareAlt } from "@fortawesome/free-solid-svg-icons";
    import TableContainer from "../../components/tables/TableContainer.svelte";
    import { isEntitled } from "../../data/billing/entitlement";
    import MissingEntitlementTooltip from "../../components/billing/entitlement/MissingEntitlementTooltip.svelte";
    import { getUserId } from "../../data/auth";

    const orgCtx = getOrgContext();
    const entitled = isEntitled("user_count", true);

    let currentUserId: string | undefined = undefined;
    $: getUserId().then((u) => (currentUserId = u));

    let members: APIOrgMember[] = [];
    let membersLoading = true;
    $: APIs.orgMembers()
        .then((a) => a.organisationMembersList($orgCtx.org.id))
        .then((resp) => {
            members = resp.data;
            membersLoading = false;
        });

    $: onMemberDelete = (id: string) => {
        members = members.filter((e) => e.user_id !== id);
    };
    $: onMemberUpdate = (id: string, data: APIOrgMember) => {
        const i = members.findIndex((e) => e.user_id === id);
        members[i] = data;
    };
</script>

<Button
    href={`/orgs/${$orgCtx.org.id}/settings/members/invite`}
    on:click={navigateEvent}
    disabled={!$entitled || $orgCtx.org.uses_oidc}
    outline
>
    <FontAwesomeIcon icon={faShareAlt} class="me-2" />
    Invite members
</Button>
<MissingEntitlementTooltip key="user_count" multi />
{#if $entitled && $orgCtx.org.uses_oidc}
    <Tooltip placement="right">
        Cannot manually invite members when your organisation uses OIDC login
    </Tooltip>
{/if}

{#if membersLoading}
    <div class="text-center">
        <Spinner size={14} />
    </div>
{:else}
    <TableContainer class="mt-4">
        <Table divClass="">
            <TableHead>
                <TableHeadCell>Display name</TableHeadCell>
                <TableHeadCell>User ID</TableHeadCell>
                <TableHeadCell>Organisation Admin</TableHeadCell>
                <TableHeadCell>
                    <span class="sr-only">Actions</span>
                </TableHeadCell>
            </TableHead>
            <TableBody>
                {#each members as member (member.user_id)}
                    <OrganisationMemberRow
                        {member}
                        on:delete={() => onMemberDelete(member.user_id)}
                        on:update={(e) =>
                            onMemberUpdate(member.user_id, e.detail)}
                        isSelf={currentUserId == member.user_id}
                    />
                {/each}
            </TableBody>
        </Table>
    </TableContainer>
{/if}
