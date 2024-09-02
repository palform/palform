<script lang="ts">
    import { Alert, Button, Label, Modal, Select } from "flowbite-svelte";
    import { Link } from "svelte-routing";
    import {
        getFormCtx,
        getOrgContext,
        updateFormCtx,
    } from "../../../data/contexts/orgLayout";
    import InfoText from "../../type/InfoText.svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";

    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();
    let showModal = false;
    let selectedTeam = "";
    $: selectItems = $orgCtx.myTeams
        .filter(
            (e) =>
                e.team_id !== $formCtx.team_id &&
                (e.my_role === "Editor" || e.my_role === "Admin"),
        )
        .map((t) => ({
            name: t.name,
            value: t.team_id,
        }));

    let loading = false;
    $: onMoveClick = async () => {
        if (selectedTeam === "") return;
        loading = true;
        try {
            await APIs.forms().then((a) =>
                a.formsRelocate($orgCtx.org.id, $formCtx.id, selectedTeam),
            );
            await showSuccessToast("Form moved");
            updateFormCtx(orgCtx, $formCtx.id, (ctx) => {
                ctx.team_id = selectedTeam;
            });
            showModal = false;
            selectedTeam = "";
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
</script>

<Alert color="yellow" class={$$props.class}>
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
        To create or edit teams, visit your <Link
            to={`/orgs/${$orgCtx.org.id}/settings/teams`}
            class="underline"
        >
            Teams page</Link
        >.
    </p>

    <Button
        class="mt-2"
        color="red"
        outline
        on:click={() => (showModal = true)}
    >
        Move
    </Button>
</Alert>

<Modal bind:open={showModal} outsideclose title="Move form to other team">
    {#if selectItems.length === 0}
        <InfoText>There are no eligible teams to move this form to.</InfoText>
        <InfoText>
            You must have at least <strong>Editor</strong> permission in the target
            team.
        </InfoText>
    {:else}
        <Label>
            Team to move to
            <Select
                class="mt-1"
                items={selectItems}
                bind:value={selectedTeam}
                disabled={loading}
            />
        </Label>
    {/if}

    <svelte:fragment slot="footer">
        {#if selectItems.length === 0}
            <Button on:click={() => (showModal = false)}>Close</Button>
        {:else}
            <LoadingButton
                disabled={selectedTeam === "" || loading}
                {loading}
                on:click={onMoveClick}
            >
                Move
            </LoadingButton>
        {/if}
    </svelte:fragment>
</Modal>
