<script lang="ts">
    import type { APIFormBranding } from "@palform/palform-typescript-openapi";
    import CardBox from "../../cardBox/CardBox.svelte";
    import CardBoxTitle from "../../cardBox/CardBoxTitle.svelte";
    import CardBoxSubtitle from "../../cardBox/CardBoxSubtitle.svelte";
    import { Button } from "flowbite-svelte";
    import BrandingConfigModal from "./BrandingConfigModal.svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { getTeamCtx } from "../../../data/contexts/team";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import BrandingAccessModal from "./BrandingAccessModal.svelte";

    interface Props {
        branding: APIFormBranding;
    }

    let { branding }: Props = $props();
    const orgCtx = getOrgContext();
    const teamCtx = getTeamCtx();
    let showEditModal = $state(false);

    let isTeamAdmin = $derived(
        $orgCtx.myTeams.find((e) => e.team_id === $teamCtx.team.id)?.my_role ===
            "Admin"
    );

    let deleteLoading = $state(false);
    let onDeleteClick = $derived(async () => {
        deleteLoading = true;
        try {
            await APIs.formBrandings().then((a) =>
                a.organisationTeamBrandingDelete(
                    $orgCtx.org.id,
                    $teamCtx.team.id,
                    branding.id
                )
            );
            await showSuccessToast(
                "Branding scheme deleted; all existing forms reset to default branding"
            );
            teamCtx.update((ctx) => {
                return {
                    ...ctx,
                    brandings: ctx.brandings.filter(
                        (e) => e.id !== branding.id
                    ),
                };
            });
        } catch (e) {
            await showFailureToast(e);
        }
        deleteLoading = false;
    });

    let showAccessModal = $state(false);
</script>

<BrandingConfigModal
    existingBranding={branding}
    bind:modalOpen={showEditModal}
/>

<BrandingAccessModal bind:open={showAccessModal} {branding} />

<CardBox>
    <CardBoxTitle>
        {branding.name}
    </CardBoxTitle>
    <CardBoxSubtitle>Subtitle example</CardBoxSubtitle>

    <div class="mt-2">
        <Button
            outline
            onclick={() => (showEditModal = true)}
            size="xs"
            disabled={deleteLoading}
        >
            Edit
        </Button>

        <LoadingButton
            outline
            color="red"
            size="xs"
            loading={deleteLoading}
            disabled={deleteLoading}
            onclick={onDeleteClick}
        >
            Delete
        </LoadingButton>

        {#if isTeamAdmin}
            <Button
                outline
                onclick={() => (showAccessModal = true)}
                size="xs"
                disabled={deleteLoading}
            >
                Manage access
            </Button>
        {/if}
    </div>
</CardBox>
