<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import type { APIFormBrandingAccess } from "@palform/palform-typescript-openapi";
    import { Button, Label, Modal } from "flowbite-svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { getTeamCtx } from "../../../data/contexts/team";
    import TeamDropdown from "../TeamDropdown.svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";

    interface Props {
        brandingId: string;
        existingTeams: string[];
        oncreate: (access: APIFormBrandingAccess) => void;
    }

    let { brandingId, existingTeams, oncreate }: Props = $props();
    const orgCtx = getOrgContext();
    const teamCtx = getTeamCtx();

    let showModal = $state(false);

    let teamId = $state("");
    let addLoading = $state(false);

    let onAddClick = $derived(async () => {
        if (teamId === "") {
            await showFailureToast("Please select a team");
            return;
        }

        addLoading = true;
        try {
            const resp = await APIs.formBrandings().then((a) =>
                a.organisationTeamBrandingAddAccess(
                    $orgCtx.org.id,
                    $teamCtx.team.id,
                    brandingId,
                    {
                        for_team_id: teamId,
                    }
                )
            );
            oncreate(resp.data);
            await showSuccessToast(`Added access for ${resp.data.team_name}`);
            showModal = false;
        } catch (e) {
            await showFailureToast(e);
        }
        addLoading = false;
    });
</script>

<Button onclick={() => (showModal = true)}>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    Add access
</Button>

<Modal title="Add access for team" bind:open={showModal} outsideclose>
    <p>
        You need to be an admin in both the current and target team. Only
        applicable teams are shown.
    </p>

    <Label>
        <TeamDropdown
            bind:value={teamId}
            disabled={addLoading}
            hideTeams={[$teamCtx.team.id, ...existingTeams]}
            withRoleOnly={"Admin"}
        />
    </Label>

    <LoadingButton
        disabled={addLoading}
        loading={addLoading}
        onclick={onAddClick}
    >
        Add
    </LoadingButton>
</Modal>
