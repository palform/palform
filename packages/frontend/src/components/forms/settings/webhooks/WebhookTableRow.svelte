<script lang="ts">
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { TableBodyCell, TableBodyRow, Modal } from "flowbite-svelte";
    import { parseServerTime } from "../../../../data/util/time";
    import { DateTime } from "luxon";
    import TableSingleAction from "../../../tables/TableSingleAction.svelte";
    import type { APIWebhook } from "@paltiverse/palform-typescript-openapi";
    import {
        faCheckCircle,
        faExclamationCircle,
    } from "@fortawesome/free-solid-svg-icons";
    import { APIs } from "../../../../data/common";
    import {
        getFormCtx,
        getOrgContext,
    } from "../../../../data/contexts/orgLayout";
    import { createEventDispatcher } from "svelte";
    import { showFailureToast, showSuccessToast } from "../../../../data/toast";
    import WebhookJobs from "./WebhookJobs.svelte";

    export let webhook: APIWebhook;
    const orgCtx = getOrgContext();
    const formCtx = getFormCtx();
    const dispatch = createEventDispatcher<{ delete: undefined }>();

    let loading = false;
    let showModal = false;

    $: onDelete = async () => {
        loading = true;
        try {
            await APIs.webhooks().then((a) =>
                a.webhooksDelete($orgCtx.org.id, $formCtx.id, webhook.id)
            );
            await showSuccessToast(
                "Webhook deleted. That endpoint will no longer receive data."
            );
            dispatch("delete");
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
</script>

<Modal bind:open={showModal} outsideclose title="Webhook runs" size="lg">
    <WebhookJobs {webhook} />
</Modal>

<TableBodyRow>
    <TableBodyCell>
        {webhook.endpoint}
    </TableBodyCell>
    <TableBodyCell>
        <TableSingleAction on:click={() => (showModal = true)}>
            {#if webhook.is_healthy}
                <span class="text-green-500">
                    <FontAwesomeIcon class="me-1" icon={faCheckCircle} />
                    Healthy
                </span>
            {:else}
                <span class="text-red-500">
                    <FontAwesomeIcon class="me-1" icon={faExclamationCircle} />
                    Unhealthy
                </span>
            {/if}
        </TableSingleAction>
    </TableBodyCell>
    <TableBodyCell
        title={parseServerTime(webhook.created_at).toLocaleString(
            DateTime.DATETIME_MED
        )}
    >
        {parseServerTime(webhook.created_at).toRelative()}
    </TableBodyCell>
    <TableBodyCell>
        <TableSingleAction on:click={onDelete} disabled={loading}>
            Delete
        </TableSingleAction>
    </TableBodyCell>
</TableBodyRow>
