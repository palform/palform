<script lang="ts">
    import type { APIUserKey } from "@paltiverse/palform-typescript-openapi";
    import {
        DropdownItem,
        TableBodyCell,
        TableBodyRow,
        Tooltip,
    } from "flowbite-svelte";
    import type { CryptoKeyRecord } from "../../data/pouch";
    import { DateTime } from "luxon";
    import TableActions from "../tables/TableActions.svelte";
    import { parseServerTime } from "../../data/util/time";
    import { APIs } from "../../data/common";
    import { createEventDispatcher } from "svelte";
    import { showFailureToast, showSuccessToast } from "../../data/toast";
    import {
        KeyMetadata,
        get_key_metadata_js,
    } from "@paltiverse/palform-crypto";
    import {
        deleteLocalKey,
        downloadPrivateKey,
        findKey,
    } from "../../data/crypto/keyManager";
    import {
        getOrgContext,
        reloadGlobalAlert,
    } from "../../data/contexts/orgLayout";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";

    export let key: APIUserKey;
    const orgCtx = getOrgContext();
    let matchedLocalKey: CryptoKeyRecord | null | undefined = undefined;
    let keyMetadata: KeyMetadata | undefined = undefined;
    findKey(key.id).then((resp) => {
        matchedLocalKey = resp;
    });

    (async () => {
        keyMetadata = get_key_metadata_js(key.key_pem);
    })();

    const dispatch = createEventDispatcher<{ deleted: undefined }>();

    let loading = false;
    $: onDelete = async () => {
        loading = true;
        try {
            await APIs.keys().then((a) => a.keysDelete($orgCtx.org.id, key.id));
            if (matchedLocalKey) {
                await deleteLocalKey(matchedLocalKey);
                await reloadGlobalAlert(orgCtx);
            }

            dispatch("deleted");
            await showSuccessToast("Key deleted!");
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };

    const onDownload = () => {
        if (!matchedLocalKey) return;
        downloadPrivateKey(matchedLocalKey);
    };

    let createdAt = parseServerTime(key.created_at);
    let expiresAt = parseServerTime(key.expires_at);
    $: expired = expiresAt < DateTime.now();
</script>

<TableBodyRow>
    <TableBodyCell>
        <span class={expired ? "line-through text-red-600" : "text-green-600"}>
            {keyMetadata?.fingerprint}
        </span>
        {#if keyMetadata?.algo !== "ECDH"}
            <span class="block text-xs">
                {keyMetadata?.algo}
            </span>
        {/if}
    </TableBodyCell>
    <TableBodyCell>
        {#if matchedLocalKey !== undefined}
            {#if matchedLocalKey === null}
                <strong class="underline decoration-dotted">No</strong>
                <Tooltip>
                    Your don't have the private key locally available. You can
                    restore it from a backup if you have one.
                </Tooltip>
            {:else}
                Yes
            {/if}
        {/if}
    </TableBodyCell>
    <TableBodyCell>
        {key.has_backup ? "Yes" : "No"}
    </TableBodyCell>
    <TableBodyCell title={createdAt.toLocaleString(DateTime.DATETIME_MED)}>
        {createdAt.toRelative()}
    </TableBodyCell>
    <TableBodyCell title={expiresAt.toLocaleString(DateTime.DATETIME_MED)}>
        <span class={expired ? "font-bold" : ""}>
            {#if expired}
                Expired
            {/if}
            {#if expiresAt.diffNow("years").years > 80}
                Never
            {:else}
                {expiresAt.toRelative()}
            {/if}
        </span>
    </TableBodyCell>
    <TableBodyCell>
        <TableActions>
            <DropdownItem disabled={loading} on:click={onDelete}>
                Delete
            </DropdownItem>
            {#if matchedLocalKey}
                <DropdownItem disabled={loading} on:click={onDownload}>
                    Download private key
                </DropdownItem>
            {/if}
            <DropdownItem
                disabled={loading}
                href={`/orgs/${$orgCtx.org.id}/user/keys/${key.id}/backup`}
                on:click={navigateEvent}
            >
                Manage backup
            </DropdownItem>
        </TableActions>
    </TableBodyCell>
</TableBodyRow>
