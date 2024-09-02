<script lang="ts">
    import { Alert } from "flowbite-svelte";

    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showSuccessToast } from "../../../data/toast";
    import { navigate } from "svelte-routing";
    import DangerZone from "../../type/DangerZone.svelte";
    import BackupRecover from "./BackupRecover.svelte";
    import type { APIUserKey } from "@paltiverse/palform-typescript-openapi";
    import SectionSeparator from "../../type/SectionSeparator.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faLock } from "@fortawesome/free-solid-svg-icons";

    export let key: APIUserKey;
    export let privateKeyExistsLocally: boolean;
    const orgCtx = getOrgContext();

    let deleteLoading = false;
    $: onDeleteClick = async () => {
        deleteLoading = true;
        await APIs.keys().then((a) =>
            a.keysRegisterBackup($orgCtx.org.id, key.id, { key_data: null }),
        );
        deleteLoading = false;
        await showSuccessToast("Key backup deleted");
        navigate(`/orgs/${$orgCtx.org.id}/user/keys`);
    };
</script>

<Alert class="mt-4">
    <span slot="icon">
        <FontAwesomeIcon icon={faLock} />
    </span>
    Your key is currently backed up securely.
</Alert>

{#if !privateKeyExistsLocally}
    <BackupRecover {key} />
{/if}

<SectionSeparator />
<DangerZone />

{#if !privateKeyExistsLocally}
    <Alert color="red" border class="mt-4">
        <h2 class="text-lg">We don't recommend deleting your backup.</h2>
        <p>
            You currently don't have a version of the private key stored locally
            in your browser. Make sure you won't lose your private key if you
            delete this backup.
        </p>
    </Alert>
{/if}

<LoadingButton
    disabled={deleteLoading}
    loading={deleteLoading}
    on:click={onDeleteClick}
    color="red"
    buttonClass="mt-2"
>
    Delete backup
</LoadingButton>
