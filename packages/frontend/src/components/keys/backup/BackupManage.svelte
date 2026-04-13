<script lang="ts">
    import { Alert } from "flowbite-svelte";

    import LoadingButton from "../../LoadingButton.svelte";
    import { APIs } from "../../../data/common";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { showSuccessToast } from "../../../data/toast";
    import { navigate } from "../../../router";
    import DangerZone from "../../type/DangerZone.svelte";
    import BackupRecover from "./BackupRecover.svelte";
    import type { APIUserKey } from "@palform/palform-typescript-openapi";
    import SectionSeparator from "../../type/SectionSeparator.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faLock } from "@fortawesome/free-solid-svg-icons";

    interface Props {
        key: APIUserKey;
        privateKeyExistsLocally: boolean;
    }

    let { key, privateKeyExistsLocally }: Props = $props();
    const orgCtx = getOrgContext();

    let deleteLoading = $state(false);
    let onDeleteClick = $derived(async () => {
        deleteLoading = true;
        await APIs.keys().then((a) =>
            a.keysRegisterBackup($orgCtx.org.id, key.id, { key_data: null })
        );
        deleteLoading = false;
        await showSuccessToast("Key backup deleted");
        navigate(`/orgs/${$orgCtx.org.id}/user/keys`);
    });
</script>

<Alert class="mt-4">
    {#snippet icon()}
        <span>
            <FontAwesomeIcon icon={faLock} />
        </span>
    {/snippet}
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
    onclick={onDeleteClick}
    color="red"
    buttonClass="mt-2"
>
    Delete backup
</LoadingButton>
