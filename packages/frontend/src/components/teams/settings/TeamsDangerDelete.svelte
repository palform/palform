<script lang="ts">
    import { Alert, Tooltip } from "flowbite-svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { getTeamCtx } from "../../../data/contexts/team";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { navigate } from "svelte-routing";

    const orgCtx = getOrgContext();
    const teamCtx = getTeamCtx();
    let loading = false;

    $: onDeleteClick = async () => {
        loading = true;

        try {
            await APIs.orgTeams().then((a) =>
                a.organisationTeamsDelete($orgCtx.org.id, $teamCtx.team.id),
            );
            await showSuccessToast("Team deleted");
            orgCtx.update((ctx) => {
                return {
                    ...ctx,
                    myTeams: ctx.myTeams.filter(
                        (e) => e.team_id !== $teamCtx.team.id,
                    ),
                };
            });
            navigate(`/orgs/${$orgCtx.org.id}/settings/teams/`);
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    };
</script>

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
        on:click={onDeleteClick}
    >
        Delete team and all forms
    </LoadingButton>

    {#if $teamCtx.team.is_default}
        <Tooltip>Cannot delete the default team</Tooltip>
    {/if}
</Alert>
