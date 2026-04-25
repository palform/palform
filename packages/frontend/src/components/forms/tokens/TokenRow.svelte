<script lang="ts">
    import type { APIFillToken } from "@palform/palform-typescript-openapi";
    import { Modal, TableBodyCell, TableBodyRow } from "flowbite-svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showSuccessToast } from "../../../data/toast";
    import { APIs } from "../../../data/common";
    import { parseServerTime } from "../../../data/util/time";
    import TableSingleAction from "../../tables/TableSingleAction.svelte";
    import { DateTime } from "luxon";
    import TokenEmbedOptions from "./TokenEmbedOptions.svelte";
    import { getFormAdminContext } from "../../../data/contexts/formAdmin";

    interface Props {
        token: APIFillToken;
        ondelete: () => void;
    }

    let { token, ondelete }: Props = $props();

    const orgCtx = getOrgContext();
    const formAdminCtx = getFormAdminContext();

    let createdAt = $derived(parseServerTime(token.created_at));
    let expiresAt = $derived(
        token.expires_at ? parseServerTime(token.expires_at) : undefined
    );
    let expired = $derived(
        expiresAt !== undefined && expiresAt < DateTime.now()
    );

    let deleteLoading = $state(false);
    let onDeleteClick = $derived(async (id: string) => {
        deleteLoading = true;
        await APIs.fillTokens().then((a) =>
            a.fillAccessTokensDelete($orgCtx.org.id, $formAdminCtx.formId, id)
        );
        ondelete();
        deleteLoading = false;

        await showSuccessToast(
            "Token deleted! Anyone with that link can no longer fill in your form."
        );
    });

    let showViewLinkModal = $state(false);
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
            onclick={() => (showViewLinkModal = true)}
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
            onclick={() => onDeleteClick(token.id)}
        >
            Delete
        </TableSingleAction>
    </TableBodyCell>
</TableBodyRow>
