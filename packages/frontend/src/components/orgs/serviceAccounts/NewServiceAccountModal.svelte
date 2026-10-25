<script lang="ts">
    import { Alert, Button, Input, Modal } from "flowbite-svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { APIs } from "../../../data/common";
    import type { APIServiceAccountAdminUser } from "@palform/palform-typescript-openapi";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";

    interface Props {
        open: boolean;
        onnewuser: (adminUser: APIServiceAccountAdminUser) => void;
    }

    let { open = $bindable(), onnewuser }: Props = $props();

    const orgCtx = getOrgContext();
    let loading = $state(false);
    let newAdminUser = $state<APIServiceAccountAdminUser>();
    let oncreate = $derived(() => {
        loading = true;
        APIs.adminUsers()
            .then((a) => a.adminUsersServiceAccountsCreate($orgCtx.org.id))
            .then((resp) => {
                newAdminUser = resp.data;
                onnewuser(resp.data);
                showSuccessToast("New service account created");
            })
            .catch((e) => showFailureToast(e))
            .finally(() => {
                loading = false;
            });
    });
</script>

<Modal bind:open title="Add new service account">
    {#if !newAdminUser}
        <p>
            This will create a new service account and add it to your <strong
                >Default team.</strong
            >
            You can change this later.
        </p>
    {:else}
        <Alert color="green">
            Your service account has been created. It is <strong
                >not an admin</strong
            >
            of your organisation and has been added to your
            <strong>Default team</strong>
            with view-only permissions. Feel free to change this.
        </Alert>
        <Alert color="red">
            Please copy the token now, <strong>
                as this is the only time you'll see it.
            </strong>
        </Alert>

        <Input readonly value={newAdminUser.service_account_auth_token ?? ""} />
    {/if}

    {#snippet footer()}
        {#if !newAdminUser}
            <Button onclick={oncreate} {loading} disabled={loading}
                >Create</Button
            >
        {:else}
            <Button onclick={() => (open = false)}>Close</Button>
        {/if}
    {/snippet}
</Modal>
