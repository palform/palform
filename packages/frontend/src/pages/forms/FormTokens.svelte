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
    import TokenRow from "../../components/forms/tokens/TokenRow.svelte";
    import { getFormAdminContext } from "../../data/contexts/formAdmin";

    const formAdminCtx = getFormAdminContext();

    $: insertNewToken = (newToken: CustomEvent<APIFillToken>) => {
        $formAdminCtx.tokens = [...$formAdminCtx.tokens, newToken.detail];
    };

    $: onTokenDelete = (id: string) => {
        $formAdminCtx.tokens = $formAdminCtx.tokens.filter((e) => e.id !== id);
    };
</script>

<NewTokenModal on:newToken={insertNewToken} />

{#if $formAdminCtx.tokens.length === 0}
    <Alert color="blue">
        <p>This form hasn't been published yet</p>
        <p>
            You'll need to create a Share Token to let people fill your form in.
            This will give you a URL you can easily distribute.
        </p>
    </Alert>
{:else}
    <Table shadow>
        <TableHead>
            <TableHeadCell>Link</TableHeadCell>
            <TableHeadCell>Created</TableHeadCell>
            <TableHeadCell>Expires</TableHeadCell>
            <TableHeadCell>
                <span class="sr-only">Actions</span>
            </TableHeadCell>
        </TableHead>
        <TableBody>
            {#each $formAdminCtx.tokens as token (token.id)}
                <TokenRow {token} on:delete={() => onTokenDelete(token.id)} />
            {/each}
        </TableBody>
    </Table>
{/if}
