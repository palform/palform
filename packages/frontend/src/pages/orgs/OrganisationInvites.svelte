<script lang="ts">
    import type { APIOrganisationInvite } from "@palform/palform-typescript-openapi";
    import { APIs } from "../../data/common";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import {
        Alert,
        Button,
        Spinner,
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import OrganisationInviteModal from "../../components/orgs/invites/OrganisationInviteModal.svelte";
    import OrganisationInviteRow from "../../components/orgs/invites/OrganisationInviteRow.svelte";
    import { copyOrgInviteLink } from "../../data/orgInvites";
    import InfoText from "../../components/type/InfoText.svelte";
    import TableContainer from "../../components/tables/TableContainer.svelte";
    import { isEntitled } from "../../data/billing/entitlement";
    import { p } from "../../router";
    const orgCtx = getOrgContext();

    let invitesLoading = $state(true);
    let invites: APIOrganisationInvite[] = $state([]);
    const entitled = isEntitled("user_count", true);
    $effect(() => {
        APIs.orgInvites()
            .then((a) => a.organisationInvitesList($orgCtx.org.id))
            .then((resp) => {
                invites = resp.data;
                invitesLoading = false;
            });
    });

    let newModalOpen = $state(false);
    let onNewInvite = $derived(async (e: APIOrganisationInvite) => {
        newModalOpen = false;
        invites = [e, ...invites];
        await copyOrgInviteLink($orgCtx.org.id, e.id);
    });

    let onInviteDelete = $derived((id: string) => {
        invites = invites.filter((e) => e.id !== id);
    });
</script>

{#if invitesLoading}
    <div class="text-center">
        <Spinner size="4" />
    </div>
{/if}

<OrganisationInviteModal bind:open={newModalOpen} oncreate={onNewInvite} />

{#if !invitesLoading && invites.length === 0}
    <Alert border>
        <h2 class="text-lg">Invite someone to your organisation</h2>
        <p>
            Users will need to make their own Palform account and then use your
            link to join your organisation.
        </p>
        <p>
            You can make as many links as you'd like. New users will be added as
            Viewers to your default team, and you can then manually assign them
            additional roles.
        </p>
        <p>Links can be made single-use and must have an expiry date.</p>

        {#if $entitled}
            <Button class="mt-4" outline onclick={() => (newModalOpen = true)}>
                Invite someone!
            </Button>
        {:else}
            <Button
                class="mt-4"
                href={p("/orgs/:orgId/settings/billing", {
                    params: { orgId: $orgCtx.org.id },
                })}
            >
                Upgrade to continue
            </Button>
        {/if}
    </Alert>
{/if}

{#if invites.length > 0}
    <InfoText>
        These are invite links created by admins in your organisation. They can
        be used to join your organisation.
    </InfoText>

    <InfoText>
        New users will automatically be added to your <a
            href={`/orgs/${$orgCtx.org.id}/settings/teams`}
            class="font-bold hover:underline">default team</a
        >. You can manually add them to other teams once they join.
    </InfoText>

    <Button class="mt-4" onclick={() => (newModalOpen = true)}>
        New invite
    </Button>

    <TableContainer class="mt-4">
        <Table>
            <TableHead>
                <TableHeadCell>ID (copy link)</TableHeadCell>
                <TableHeadCell>Single use?</TableHeadCell>
                <TableHeadCell>Created</TableHeadCell>
                <TableHeadCell>Expires</TableHeadCell>
                <TableHeadCell>
                    <span class="sr-only">Actions</span>
                </TableHeadCell>
            </TableHead>
            <TableBody>
                {#each invites as invite (invite.id)}
                    <OrganisationInviteRow
                        {invite}
                        ondelete={() => onInviteDelete(invite.id)}
                    />
                {/each}
            </TableBody>
        </Table>
    </TableContainer>
{/if}
