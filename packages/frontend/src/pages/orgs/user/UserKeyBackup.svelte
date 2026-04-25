<script lang="ts">
    import { Alert, Button, Spinner } from "flowbite-svelte";
    import MainTitle from "../../../layouts/MainTitle.svelte";
    import { findKey } from "../../../data/crypto/keyManager";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import type { APIUserKey } from "@palform/palform-typescript-openapi";
    import { APIs } from "../../../data/common";
    import BackupNew from "../../../components/keys/backup/BackupNew.svelte";
    import BackupManage from "../../../components/keys/backup/BackupManage.svelte";
    import { navigate, p, route } from "../../../router";

    interface Props {
        keyId?: string | undefined;
    }

    let { keyId }: Props = $props();
    let { keyId: keyIdRoute } = route.getParams(
        "/orgs/:orgId/user/keys/:keyId/backup"
    );
    const keyIdResolved = $derived(keyId ?? keyIdRoute ?? "");

    let key: APIUserKey | undefined = $state();
    const orgCtx = getOrgContext();
    let loading = $state(true);
    $effect(() => {
        APIs.keys()
            .then((a) => a.keysGet($orgCtx.org.id, keyIdResolved))
            .then((resp) => {
                loading = false;
                key = resp.data;
            });
    });

    let privateKeyExistsLocally: boolean | undefined = $state(undefined);
    $effect(() => {
        const k = keyIdResolved;
        findKey(k).then((resp) => {
            privateKeyExistsLocally = resp !== null;
        });
    });

    const isNew = new URLSearchParams(location.search).get("isNew") === "y";

    const onNewBackupCreate = () => {
        navigate(`/orgs/${$orgCtx.org.id}/user/keys`);
    };
</script>

<Button
    href={p("/orgs/:orgId/user/keys", { params: { orgId: $orgCtx.org.id } })}
    size="xs"
    class="mb-4"
    outline
>
    Cancel
</Button>
<MainTitle className="mb-4">Backup your key</MainTitle>

{#if loading || privateKeyExistsLocally === undefined}
    <div class="text-center mt-4">
        <Spinner size="12" />
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
            <BackupNew keyId={keyIdResolved} ondone={onNewBackupCreate} />
        {/if}
    {:else}
        <BackupManage {privateKeyExistsLocally} {key} />
    {/if}
{/if}
