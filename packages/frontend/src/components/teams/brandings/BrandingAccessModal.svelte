<script lang="ts">
    import type {
        APIFormBranding,
        APIFormBrandingAccess,
    } from "@palform/palform-typescript-openapi";
    import {
        Modal,
        Spinner,
        Table,
        TableBody,
        TableBodyCell,
        TableBodyRow,
    } from "flowbite-svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { getTeamCtx } from "../../../data/contexts/team";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import TableContainer from "../../tables/TableContainer.svelte";
    import TableSingleAction from "../../tables/TableSingleAction.svelte";
    import BrandingAccessNewModal from "./BrandingAccessNewModal.svelte";

    interface Props {
        open?: boolean;
        branding: APIFormBranding;
    }

    let { open = $bindable(false), branding }: Props = $props();
    const orgCtx = getOrgContext();
    const teamCtx = getTeamCtx();

    let initLoading = $state(true);
    let access: APIFormBrandingAccess[] | undefined = $state(undefined);
    APIs.formBrandings()
        .then((a) =>
            a.organisationTeamBrandingListAccess(
                $orgCtx.org.id,
                $teamCtx.team.id,
                branding.id
            )
        )
        .then((resp) => {
            access = resp.data;
        })
        .catch((e) => showFailureToast(e))
        .finally(() => (initLoading = false));

    let onNewAccess = $derived((e: APIFormBrandingAccess) => {
        if (access === undefined) return;
        access = [e, ...access];
    });

    let deleteLoading = $state(false);
    let onDeleteClick = $derived(async (teamId: string) => {
        if (access === undefined) return;

        deleteLoading = true;
        try {
            await APIs.formBrandings().then((a) =>
                a.organisationTeamBrandingDeleteAccess(
                    $orgCtx.org.id,
                    $teamCtx.team.id,
                    branding.id,
                    { for_team_id: teamId }
                )
            );
            await showSuccessToast("Access removed");
            access = access.filter((e) => e.team_id !== teamId);
        } catch (e) {
            await showFailureToast(e);
        }
        deleteLoading = false;
    });
</script>

<Modal bind:open outsideclose title={`Sharing for ${branding.name}`}>
    {#if initLoading}
        <Spinner />
    {/if}

    {#if access !== undefined}
        <p>
            These teams have access to this branding scheme and can use it in
            forms.
        </p>

        <TableContainer>
            <Table divClass="" striped>
                <TableBody>
                    {#each access as entry (entry.team_id)}
                        <TableBodyRow>
                            <TableBodyCell>
                                {entry.team_name}

                                {#if entry.team_id === $teamCtx.team.id}
                                    <span
                                        class="block text-xs text-gray-600 dark:text-gray-400"
                                    >
                                        (this team)
                                    </span>
                                {/if}
                            </TableBodyCell>
                            <TableBodyCell class="text-right">
                                {#if entry.team_id !== $teamCtx.team.id}
                                    <TableSingleAction
                                        onclick={() =>
                                            onDeleteClick(entry.team_id)}
                                        disabled={deleteLoading}
                                    >
                                        Remove access
                                    </TableSingleAction>
                                {/if}
                            </TableBodyCell>
                        </TableBodyRow>
                    {/each}
                </TableBody>
            </Table>
        </TableContainer>

        <BrandingAccessNewModal
            brandingId={branding.id}
            oncreate={onNewAccess}
            existingTeams={access.map((e) => e.team_id)}
        />
    {/if}
</Modal>
