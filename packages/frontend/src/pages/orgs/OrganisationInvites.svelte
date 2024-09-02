<script lang="ts">
    import type { APIOrganisationInvite } from "@paltiverse/palform-typescript-openapi";
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
    import { Link } from "svelte-routing";

    const orgCtx = getOrgContext();

    let invitesLoading = true;
    let invites: APIOrganisationInvite[] = [];
    $: APIs.orgInvites()
        .then((a) => a.organisationInvitesList($orgCtx.org.id))
        .then((resp) => {
            invites = resp.data;
            invitesLoading = false;
        });

    let newModalOpen = false;
    $: onNewInvite = async (e: CustomEvent<APIOrganisationInvite>) => {
        newModalOpen = false;
        invites = [e.detail, ...invites];
        await copyOrgInviteLink($orgCtx.org.id, e.detail.id);
    };

    $: onInviteDelete = (id: string) => {
        invites = invites.filter((e) => e.id !== id);
    };
</script>

{#if invitesLoading}
    <div class="text-center">
        <Spinner size={14} />
    </div>
{/if}

<OrganisationInviteModal bind:open={newModalOpen} on:create={onNewInvite} />

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

        <Button class="mt-4" outline on:click={() => (newModalOpen = true)}>
            Invite someone!
        </Button>
    </Alert>
{/if}

{#if invites.length > 0}
    <InfoText>
        These are invite links created by admins in your organisation. They can
        be used to join your organisation.
    </InfoText>

    <InfoText>
        New users will automatically be added to your <Link
            to={`/orgs/${$orgCtx.org.id}/settings/teams`}
            class="font-bold hover:underline">default team</Link
        >. You can manually add them to other teams once they join.
    </InfoText>

    <Button class="mt-4" on:click={() => (newModalOpen = true)}>
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
                        on:delete={() => onInviteDelete(invite.id)}
                    />
                {/each}
            </TableBody>
        </Table>
    </TableContainer>
{/if}
