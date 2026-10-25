<script lang="ts">
    import { Button, Label, Modal } from "flowbite-svelte";
    import InfoText from "../../type/InfoText.svelte";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import TeamDropdown from "../../teams/TeamDropdown.svelte";
    import {
        addFormToCtx,
        getFormCtx,
        getOrgContext,
    } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { typedNavigate } from "../../../router";
    import LoadingButton from "../../LoadingButton.svelte";

    const formMetadataCtx = getFormCtx();
    const orgCtx = getOrgContext();

    let showModal = $state(false);
    let selectedTeam = $state($formMetadataCtx.team_id);

    let loading = $state(false);
    let onCloneClick = $derived(async () => {
        if (selectedTeam === "") return;
        loading = true;
        try {
            const newForm = await APIs.forms().then((a) =>
                a.formsClone($formMetadataCtx.id, $orgCtx.org.id, {
                    new_team_id: selectedTeam,
                })
            );
            await showSuccessToast("Form cloned");
            addFormToCtx(orgCtx, newForm.data);
            showModal = false;
            await typedNavigate("/orgs/:orgId/forms/:formId/edit", {
                params: { orgId: $orgCtx.org.id, formId: newForm.data.id },
            });
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    });
</script>

<SectionHeading class="mb-4">Clone form</SectionHeading>

<InfoText lighter class="mb-2">
    You can clone this form into a team of your choice. All questions and
    settings will be retained, but the branding scheme will be reset to <strong
        >Default</strong
    >. Responses will not be copied.
</InfoText>

<Button outline onclick={() => (showModal = true)}>Clone</Button>

<Modal bind:open={showModal} title="Clone form">
    <InfoText>
        You must have at least <strong>Editor</strong> permission in the target team.
    </InfoText>
    <Label>
        Team to clone into
        <TeamDropdown
            bind:value={selectedTeam}
            required
            allowedRoles={["Editor", "Admin"]}
        />
    </Label>

    {#snippet footer()}
        <LoadingButton
            disabled={selectedTeam === "" || loading}
            {loading}
            onclick={onCloneClick}
        >
            Clone
        </LoadingButton>
        <Button onclick={() => (showModal = false)} color="light">Cancel</Button
        >
    {/snippet}
</Modal>
