<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, Helper, Input, Label, Modal } from "flowbite-svelte";
    import LoadingButton from "../LoadingButton.svelte";
    import { APIs } from "../../data/common";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../data/toast";
    import { navigate } from "../../router";
    import { isEntitled } from "../../data/billing/entitlement";
    import MissingEntitlementTooltip from "../billing/entitlement/MissingEntitlementTooltip.svelte";

    interface Props {
        class?: string;
    }

    let { class: className }: Props = $props();

    const orgCtx = getOrgContext();
    const entitled = isEntitled("team_count", true);
    let showModal = $state(false);
    let teamName = $state("");
    let loading = $state(false);

    let onCreateClick = $derived(async () => {
        loading = true;

        try {
            const resp = await APIs.orgTeams().then((a) =>
                a.organisationTeamsCreate($orgCtx.org.id, { name: teamName })
            );

            orgCtx.update((ctx) => {
                return {
                    ...ctx,
                    myTeams: [
                        ...ctx.myTeams,
                        {
                            team_id: resp.data,
                            name: teamName,
                            my_role: "Admin",
                        },
                    ],
                };
            });

            await showSuccessToast("Team created");
            navigate(
                `/orgs/${$orgCtx.org.id}/settings/teams/${resp.data}/members`
            );
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    });
</script>

<Button
    class={className}
    onclick={() => (showModal = true)}
    disabled={!$entitled}
>
    <FontAwesomeIcon icon={faPlus} class="me-2" />
    New team
</Button>
<MissingEntitlementTooltip key="team_count" multi />

<Modal bind:open={showModal} outsideclose title="New team">
    <Label>
        Team name
        <Input class="mt-1" bind:value={teamName} disabled={loading} />
        <Helper class="mt-2">
            This will be visible to everyone in {$orgCtx.org.display_name},
            including non-members
        </Helper>
    </Label>

    {#snippet footer()}
        <LoadingButton disabled={loading} {loading} onclick={onCreateClick}>
            Create
        </LoadingButton>
    {/snippet}
</Modal>
