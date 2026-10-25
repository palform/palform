<script lang="ts">
    import { Alert, Button, Label, Modal } from "flowbite-svelte";
    import {
        getFormCtx,
        getOrgContext,
        updateFormCtx,
    } from "../../../data/contexts/orgLayout";
    import InfoText from "../../type/InfoText.svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import TeamDropdown from "../../teams/TeamDropdown.svelte";

    interface Props {
        class?: string;
    }

    let { class: className }: Props = $props();

    const orgCtx = getOrgContext();
    const formMetadataCtx = getFormCtx();

    let showModal = $state(false);
    let selectedTeam = $state("");

    let loading = $state(false);
    let onMoveClick = $derived(async () => {
        if (selectedTeam === "") return;
        loading = true;
        try {
            await APIs.forms().then((a) =>
                a.formsRelocate($formMetadataCtx.id, $orgCtx.org.id, {
                    target_team_id: selectedTeam,
                })
            );
            await showSuccessToast("Form moved");
            updateFormCtx(orgCtx, $formMetadataCtx.id, (ctx) => {
                ctx.team_id = selectedTeam;
            });
            showModal = false;
            selectedTeam = "";
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    });
</script>

<Alert color="orange" class={className}>
    <h3 class="text-lg">Move form to other team</h3>

    <p>
        You can move this form to an existing team without affecting the
        existing submissions.
    </p>
    <p>
        You must have at least <strong>Editor</strong> permission in both the current
        and target teams.
    </p>
    <p>
        The branding scheme will be reset to <strong>Default</strong>, as
        branding schemes are team-specific.
    </p>
    <p>
        To create or edit teams, visit your <a
            href={`/orgs/${$orgCtx.org.id}/settings/teams`}
            class="underline"
        >
            Teams page</a
        >.
    </p>

    <Button class="mt-2" color="red" outline onclick={() => (showModal = true)}>
        Move
    </Button>
</Alert>

<Modal bind:open={showModal} outsideclose title="Move form to other team">
    <InfoText>
        You must have at least <strong>Editor</strong> permission in the target team.
    </InfoText>
    <Label>
        Team to move to
        <TeamDropdown
            bind:value={selectedTeam}
            required
            hideTeams={[$formMetadataCtx.team_id]}
            allowedRoles={["Editor", "Admin"]}
        />
    </Label>

    {#snippet footer()}
        <LoadingButton
            disabled={selectedTeam === "" || loading}
            {loading}
            onclick={onMoveClick}
            color="orange"
        >
            Move
        </LoadingButton>
        <Button onclick={() => (showModal = false)} color="light">Cancel</Button
        >
    {/snippet}
</Modal>
