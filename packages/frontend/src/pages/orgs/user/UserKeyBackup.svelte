<script lang="ts">
    import { Alert, Button, Spinner } from "flowbite-svelte";
    import MainTitle from "../../../layouts/MainTitle.svelte";
    import { findKey } from "../../../data/crypto/keyManager";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import type { APIUserKey } from "@paltiverse/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import BackupNew from "../../../components/keys/backup/BackupNew.svelte";
    import BackupManage from "../../../components/keys/backup/BackupManage.svelte";
    import { navigate } from "svelte-routing";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";

    export let keyId: string;
    let key: APIUserKey | undefined;
    const orgCtx = getOrgContext();
    let loading = true;
    $: APIs.keys()
        .then((a) => a.keysGet($orgCtx.org.id, keyId))
        .then((resp) => {
            loading = false;
            key = resp.data;
        });

    let privateKeyExistsLocally: boolean | undefined = undefined;
    findKey(keyId).then((resp) => {
        privateKeyExistsLocally = resp !== null;
    });

    const isNew = new URLSearchParams(location.search).get("isNew") === "y";

    const onNewBackupCreate = () => {
        navigate(`/orgs/${$orgCtx.org.id}/user/keys`);
    };
</script>

<Button
    href={`/orgs/${$orgCtx.org.id}/user/keys`}
    on:click={navigateEvent}
    size="xs"
    class="mb-4"
    outline
>
    Cancel
</Button>
<MainTitle>Backup your key</MainTitle>

{#if loading || privateKeyExistsLocally === undefined}
    <div class="text-center mt-4">
        <Spinner size={14} />
    </div>
{:else if key !== undefined}
    {#if isNew}
        <Alert color="green" class="mt-4">Your key has been registered.</Alert>
    {/if}

    {#if !key.has_backup}
        {#if !privateKeyExistsLocally}
            <Alert color="red" border class="mt-4">
                <h2 class="text-lg">We can't back up your key</h2>

                <p>
                    Right now, your key is not backed up, but a private key is
                    also not saved locally in your browser, so we can't create a
                    new backup.
                </p>
                <p>
                    Please try again from a browser where the private key is
                    stored locally.
                </p>
            </Alert>
        {:else}
            <BackupNew {keyId} on:done={onNewBackupCreate} />
        {/if}
    {:else}
        <BackupManage {privateKeyExistsLocally} {key} />
    {/if}
{/if}
