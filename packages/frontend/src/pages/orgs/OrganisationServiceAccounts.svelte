<script lang="ts">
    import type {
        APIOrgMemberServiceAccount,
        APIServiceAccountAdminUser,
    } from "@palform/palform-typescript-openapi";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { APIs } from "../../data/common";
    import {
        Button,
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faPlus,
        faQuestionCircle,
    } from "@fortawesome/free-solid-svg-icons";
    import TableContainer from "../../components/tables/TableContainer.svelte";
    import NewServiceAccountModal from "../../components/orgs/serviceAccounts/NewServiceAccountModal.svelte";
    import OrganisationServiceAccountRow from "../../components/orgs/serviceAccounts/OrganisationServiceAccountRow.svelte";
    import TableSkeleton from "../../components/tables/TableSkeleton.svelte";
    import ServiceAccountsIntro from "../../components/orgs/serviceAccounts/ServiceAccountsIntro.svelte";
    import { isEntitled } from "../../data/billing/entitlement";
    import SkeletonPrimitive from "../../components/SkeletonPrimitive.svelte";

    const orgCtx = getOrgContext();
    const entitled = isEntitled("service_account");

    let serviceAccounts: APIOrgMemberServiceAccount[] = $state([]);
    let serviceAccountsLoading = $state(true);
    $effect(() => {
        if (!$entitled) return;
        APIs.orgMembers()
            .then((a) =>
                a.organisationMembersListServiceAccount($orgCtx.org.id)
            )
            .then((resp) => {
                serviceAccounts = resp.data;
                serviceAccountsLoading = false;
            });
    });

    let showServiceAccountModal = $state(false);

    let onNewServiceAccount = $derived((sa: APIServiceAccountAdminUser) => {
        serviceAccounts = [
            ...serviceAccounts,
            {
                organisation_id: $orgCtx.org.id,
                user_id: sa.id,
                user_email: sa.email,
                is_admin: false,
                user_created_at: sa.created_at,
            },
        ];
    });
    let onServiceAccountDelete = $derived((id: string) => {
        serviceAccounts = serviceAccounts.filter((e) => e.user_id !== id);
    });
    let onServiceAccountUpdate = $derived(
        (id: string, data: APIOrgMemberServiceAccount) => {
            const i = serviceAccounts.findIndex((e) => e.user_id === id);
            serviceAccounts[i] = data;
        }
    );
</script>

<NewServiceAccountModal
    bind:open={showServiceAccountModal}
    onnewuser={onNewServiceAccount}
/>

{#if serviceAccountsLoading}
    <SkeletonPrimitive height="40px" width="200px" />
    <TableSkeleton class="mt-4" />
{:else if !$entitled}
    <ServiceAccountsIntro />
{:else}
    <Button onclick={() => (showServiceAccountModal = true)}>
        <FontAwesomeIcon icon={faPlus} class="me-2" />
        Add new service account
    </Button>
    <Button
        href="https://docs.palform.app/api/getting_started"
        target="_blank"
        outline
    >
        <FontAwesomeIcon icon={faQuestionCircle} class="me-2" />
        View documentation
    </Button>

    {#if serviceAccounts.length > 0}
        <TableContainer class="mt-4">
            <Table divClass="">
                <TableHead>
                    <TableHeadCell>ID</TableHeadCell>
                    <TableHeadCell>Organisation Admin</TableHeadCell>
                    <TableHeadCell>Created</TableHeadCell>
                    <TableHeadCell>
                        <span class="sr-only">Actions</span>
                    </TableHeadCell>
                </TableHead>
                <TableBody>
                    {#each serviceAccounts as member (member.user_id)}
                        <OrganisationServiceAccountRow
                            {member}
                            ondelete={() =>
                                onServiceAccountDelete(member.user_id)}
                            onupdate={(e) =>
                                onServiceAccountUpdate(member.user_id, e)}
                        />
                    {/each}
                </TableBody>
            </Table>
        </TableContainer>
    {/if}
{/if}
