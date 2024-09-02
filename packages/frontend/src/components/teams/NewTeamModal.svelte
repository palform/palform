<script lang="ts">
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, Helper, Input, Label, Modal } from "flowbite-svelte";
    import LoadingButton from "../LoadingButton.svelte";
    import { APIs } from "../../data/common";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../data/toast";
    import { navigate } from "svelte-routing";
    import { isEntitled } from "../../data/billing/entitlement";
    import MissingEntitlementTooltip from "../billing/entitlement/MissingEntitlementTooltip.svelte";

    const orgCtx = getOrgContext();
    const entitled = isEntitled("team_count", true);
    let showModal = false;
    let teamName = "";
    let loading = false;

    $: onCreateClick = async () => {
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
    };
</script>

<Button
    class={$$props.class}
    on:click={() => (showModal = true)}
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

    <svelte:fragment slot="footer">
        <LoadingButton disabled={loading} {loading} on:click={onCreateClick}>
            Create
        </LoadingButton>
    </svelte:fragment>
</Modal>
