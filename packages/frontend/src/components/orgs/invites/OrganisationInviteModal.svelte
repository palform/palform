<script lang="ts">
    import { Label, Modal, Select, Toggle } from "flowbite-svelte";
    import { type APIOrganisationInvite } from "@palform/palform-typescript-openapi";
    import expiryTimeOptions from "../../../data/util/expiryTimeOptions";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import {
        getOrgContext,
        reloadInduction,
    } from "../../../data/contexts/orgLayout";
    import { showFailureToast } from "../../../data/toast";

    interface Props {
        open?: boolean;
        oncreate?: (invite: APIOrganisationInvite) => void;
    }

    let { open = $bindable(false), oncreate }: Props = $props();

    const orgCtx = getOrgContext();

    let expiresIn: number = $state(7 * 24 * 60);
    let singleUse = $state(true);

    let loading = $state(false);
    let onCreateClick = $derived(async () => {
        loading = true;
        try {
            const resp = await APIs.orgInvites().then((a) =>
                a.organisationInvitesCreate($orgCtx.org.id, {
                    expires_in_seconds: expiresIn * 60,
                    single_use: singleUse,
                })
            );
            await reloadInduction(orgCtx);
            oncreate?.(resp.data);
        } catch (e) {
            showFailureToast(e);
        }
        loading = false;
    });
</script>

<Modal bind:open title="Create new invite link" outsideclose>
    <form class="space-y-4">
        <Label>
            Expires in
            <Select
                class="mt-2"
                items={expiryTimeOptions(false)}
                bind:value={expiresIn}
                disabled={loading}
            />
        </Label>
        <Toggle bind:checked={singleUse} disabled={loading}>Single-use</Toggle>
    </form>

    {#snippet footer()}
        <LoadingButton {loading} disabled={loading} onclick={onCreateClick}>
            Create
        </LoadingButton>
    {/snippet}
</Modal>
