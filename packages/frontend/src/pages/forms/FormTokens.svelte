<script lang="ts">
    import type { APIFillToken } from "@paltiverse/palform-typescript-openapi";
    import {
        Alert,
        Table,
        TableBody,
        TableHead,
        TableHeadCell,
    } from "flowbite-svelte";
    import NewTokenModal from "../../components/forms/tokens/NewTokenModal.svelte";
    import { getResponsesContext } from "../../data/contexts/results";
    import TokenRow from "../../components/forms/tokens/TokenRow.svelte";

    const respCtx = getResponsesContext();

    $: insertNewToken = (newToken: CustomEvent<APIFillToken>) => {
        $respCtx.tokens = [...$respCtx.tokens, newToken.detail];
    };

    $: onTokenDelete = (id: string) => {
        $respCtx.tokens = $respCtx.tokens.filter((e) => e.id !== id);
    };
</script>

<NewTokenModal on:newToken={insertNewToken} />

{#if $respCtx.tokens.length === 0}
    <Alert color="blue">
        <p>Sharing isn't set up for this form yet.</p>
        <p>
            You'll need to create a Share Token to let people fill your form in.
            This will give you a URL you can easily distribute.
        </p>
    </Alert>
{:else}
    <Table shadow>
        <TableHead>
            <TableHeadCell>Share token</TableHeadCell>
            <TableHeadCell>Created</TableHeadCell>
            <TableHeadCell>Expires</TableHeadCell>
            <TableHeadCell>
                <span class="sr-only">Actions</span>
            </TableHeadCell>
        </TableHead>
        <TableBody>
            {#each $respCtx.tokens as token (token.id)}
                <TokenRow {token} on:delete={() => onTokenDelete(token.id)} />
            {/each}
        </TableBody>
    </Table>
{/if}
