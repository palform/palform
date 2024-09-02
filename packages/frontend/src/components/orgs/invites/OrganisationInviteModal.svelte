<script lang="ts">
    import { Label, Modal, Select, Toggle } from "flowbite-svelte";
    import {
        type APIOrganisationInvite,
    } from "@paltiverse/palform-typescript-openapi";
    import expiryTimeOptions from "../../../data/util/expiryTimeOptions";
    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import {
        getOrgContext,
        reloadInduction,
    } from "../../../data/contexts/orgLayout";
    import { createEventDispatcher } from "svelte";

    export let open = false;

    const orgCtx = getOrgContext();
    const dispatch = createEventDispatcher<{ create: APIOrganisationInvite }>();

    let expiresIn: number = 7 * 24 * 60;
    let singleUse = true;

    let loading = false;
    $: onCreateClick = async () => {
        loading = true;
        const resp = await APIs.orgInvites().then((a) =>
            a.organisationInvitesCreate($orgCtx.org.id, {
                expires_in_seconds: expiresIn * 60,
                single_use: singleUse,
            }),
        );
        await reloadInduction(orgCtx);
        dispatch("create", resp.data);
        loading = false;
    };
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

    <svelte:fragment slot="footer">
        <LoadingButton {loading} disabled={loading} on:click={onCreateClick}>
            Create
        </LoadingButton>
    </svelte:fragment>
</Modal>
