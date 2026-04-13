<script lang="ts">
    import { Alert, Button, Input, Label, Modal } from "flowbite-svelte";
    import LoadingButton from "../../../LoadingButton.svelte";
    import { APIs } from "../../../../data/common";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../../data/contexts/orgLayout";
    import { showFailureToast } from "../../../../data/toast";
    import type { APIWebhook } from "@palform/palform-typescript-openapi";
    import { DateTime } from "luxon";

    interface Props {
        show?: boolean;
        oncreate: (webhook: APIWebhook) => void;
    }

    let { show = $bindable(false), oncreate }: Props = $props();
    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();

    let endpoint = $state("");
    let loading = $state(false);
    let signingSecret: string | undefined = $state(undefined);

    let onAdd = $derived(async () => {
        loading = true;
        try {
            const resp = await APIs.webhooks().then((a) =>
                a.webhooksCreate($orgCtx.org.id, $formCtx.id, {
                    endpoint,
                })
            );

            signingSecret = resp.data.signing_secret;
            oncreate({
                id: resp.data.id,
                created_at: DateTime.now().toISO(),
                endpoint,
                form_id: $formCtx.id,
                is_healthy: true,
            });
        } catch (e) {
            await showFailureToast(e);
            loading = false;
        }
    });
</script>

<Modal bind:open={show} outsideclose title="New webhook">
    {#if signingSecret}
        <Alert>
            Webhook created! Please copy the signing secret, as you won't be
            able to see it again. Your webhook will now start receiving form
            responses.
        </Alert>

        <Label>
            Signing secret
            <Input class="mt-2" value={signingSecret} readonly />
        </Label>
    {:else}
        <p>
            An HTTP POST request will be sent to this URL whenever a form
            response is made, containing the encrypted response data.
        </p>
        <p>
            Requests will be signed with a secret shown after creation. Make
            sure to verify the request's authenticity.
        </p>

        <Label>
            Endpoint URL
            <Input
                class="mt-2"
                bind:value={endpoint}
                type="url"
                disabled={loading}
            />
        </Label>
    {/if}

    {#snippet footer()}
        {#if signingSecret}
            <Button onclick={() => (show = false)}>Close</Button>
        {:else}
            <LoadingButton disabled={loading} {loading} onclick={onAdd}>
                Add
            </LoadingButton>
        {/if}
    {/snippet}
</Modal>
