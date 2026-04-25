<script lang="ts">
    import type { APIWebhook } from "@palform/palform-typescript-openapi";
    import { APIs } from "../../../../data/common";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../../data/contexts/orgLayout";
    import InfoText from "../../../type/InfoText.svelte";
    import TableContainer from "../../../tables/TableContainer.svelte";
    import {
        Alert,
        Button,
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import NewWebhook from "./NewWebhook.svelte";
    import WebhookTableRow from "./WebhookTableRow.svelte";

    interface Props {
        class?: string;
    }

    let { class: className }: Props = $props();

    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();
    let webhooks: APIWebhook[] = $state([]);

    let loading = $state(true);
    APIs.webhooks()
        .then((a) => a.webhooksList($orgCtx.org.id, $formCtx.id))
        .then((resp) => {
            webhooks = resp.data;
        })
        .finally(() => (loading = false));

    let onNew = $derived((e: APIWebhook) => {
        webhooks = [...webhooks, e];
    });

    let showNewModal = $state(false);
    let onDelete = $derived((id: string) => {
        webhooks = webhooks.filter((e) => e.id !== id);
    });
</script>

<div class={className}>
    <InfoText>Webhooks</InfoText>

    {#if !loading && webhooks.length === 0}
        <Alert class="mt-2" color="blue">
            Add a webhook to have your form responses delivered through a signed
            and encrypted HTTP message.
        </Alert>
    {/if}

    <NewWebhook bind:show={showNewModal} oncreate={onNew} />
    <Button
        outline
        class="mt-2"
        onclick={() => (showNewModal = true)}
        size="sm"
    >
        <FontAwesomeIcon icon={faPlus} class="me-2" />
        New webhook
    </Button>

    {#if webhooks.length > 0}
        <TableContainer class="mt-4">
            <Table>
                <TableHead>
                    <TableHeadCell>Endpoint</TableHeadCell>
                    <TableHeadCell>Status</TableHeadCell>
                    <TableHeadCell>Created</TableHeadCell>
                    <TableHeadCell>
                        <span class="sr-only">Actions</span>
                    </TableHeadCell>
                </TableHead>
                <TableBody>
                    {#each webhooks as webhook (webhook.id)}
                        <WebhookTableRow
                            {webhook}
                            ondelete={() => onDelete(webhook.id)}
                        />
                    {/each}
                </TableBody>
            </Table>
        </TableContainer>
    {/if}
</div>
