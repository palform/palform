<script lang="ts">
    import type { APIFillToken } from "@paltiverse/palform-typescript-openapi";
    import { Modal, TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { getResponsesContext } from "../../../data/contexts/results";
    import { showToast } from "../../../data/toast";
    import { faCheck } from "@fortawesome/free-solid-svg-icons";
    import { APIs } from "../../../data/common";
    import { createEventDispatcher } from "svelte";
    import { parseServerTime } from "../../../data/util/time";
    import TableSingleAction from "../../tables/TableSingleAction.svelte";
    import { DateTime } from "luxon";
    import TokenEmbedOptions from "./TokenEmbedOptions.svelte";

    export let token: APIFillToken;

    const orgCtx = getOrgContext();
    const respCtx = getResponsesContext();

    const dispatch = createEventDispatcher<{ delete: undefined }>();

    $: createdAt = parseServerTime(token.created_at);
    $: expiresAt = token.expires_at
        ? parseServerTime(token.expires_at)
        : undefined;
    $: expired = expiresAt !== undefined && expiresAt < DateTime.now();

    let deleteLoading = false;
    $: onDeleteClick = async (id: string) => {
        deleteLoading = true;
        await APIs.fillTokens().then((a) =>
            a.fillAccessTokensDelete($orgCtx.org.id, $respCtx.formId, id)
        );
        dispatch("delete");
        deleteLoading = false;

        await showToast({
            label: "Token deleted! Anyone with that link can no longer fill in your form.",
            color: "green",
            icon: faCheck,
        });
    };

    let showViewLinkModal = false;
</script>

<Modal
    outsideclose
    title={`View link ${token.nickname}`}
    bind:open={showViewLinkModal}
>
    <TokenEmbedOptions
        fatID={token.id}
        shortLink={token.short_link ?? undefined}
    />
</Modal>

<TableBodyRow>
    <TableBodyCell>
        {token.nickname}
        <button
            class={`block hover:underline ${expired ? "text-red-600 line-through" : "text-primary-600"}`}
            title="Copy shareable URL"
            on:click={() => (showViewLinkModal = true)}
        >
            View link
        </button>
    </TableBodyCell>
    <TableBodyCell>
        {createdAt.toRelative()}
    </TableBodyCell>
    <TableBodyCell>
        {#if !expiresAt}
            Never
        {:else}
            {#if expired}
                <strong>Expired</strong>
            {/if}
            {expiresAt.toRelative()}
        {/if}
    </TableBodyCell>
    <TableBodyCell>
        <TableSingleAction
            disabled={deleteLoading}
            on:click={() => onDeleteClick(token.id)}
        >
            Delete
        </TableSingleAction>
    </TableBodyCell>
</TableBodyRow>
