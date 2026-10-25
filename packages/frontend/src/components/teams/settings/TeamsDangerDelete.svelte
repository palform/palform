<script lang="ts">
    import { Alert, P, Tooltip } from "flowbite-svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { getTeamCtx } from "../../../data/contexts/team";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { navigate } from "../../../router";
    import DestructiveModal from "../../DestructiveModal.svelte";

    const orgCtx = getOrgContext();
    const teamCtx = getTeamCtx();
    let loading = $state(false);
    let showDeleteModal = $state(false);

    let onDeleteClick = $derived(async () => {
        loading = true;

        try {
            await APIs.orgTeams().then((a) =>
                a.organisationTeamsDelete($orgCtx.org.id, $teamCtx.team.id)
            );
            await showSuccessToast("Team deleted");
            orgCtx.update((ctx) => {
                return {
                    ...ctx,
                    myTeams: ctx.myTeams.filter(
                        (e) => e.team_id !== $teamCtx.team.id
                    ),
                };
            });
            navigate(`/orgs/${$orgCtx.org.id}/settings/teams/`);
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    });
</script>

<DestructiveModal
    bind:show={showDeleteModal}
    targetName={$teamCtx.team.name}
    confirmationWord={$teamCtx.team.name}
    ondelete={onDeleteClick}
>
    <P>
        This will <strong>irreversibly</strong> delete this team and all of its forms
        and responses. All published links to fill the forms in this team will stop
        working immediately.
    </P>
    <P>Please check what you're doing carefully to avoid any data loss.</P>
</DestructiveModal>

<Alert color="red" class="mt-4">
    <h3 class="text-lg">Delete team and owned resources</h3>
    <p>
        This will delete ALL forms owned by this team, including all responses.
    </p>
    <p>
        Please review this data to make sure nothing important is being deleted.
    </p>

    <LoadingButton
        color="red"
        outline
        buttonClass="mt-2"
        disabled={loading || ($teamCtx.team.is_default ?? false)}
        {loading}
        onclick={() => (showDeleteModal = true)}
    >
        Delete team and all forms
    </LoadingButton>

    {#if $teamCtx.team.is_default}
        <Tooltip>Cannot delete the default team</Tooltip>
    {/if}
</Alert>
