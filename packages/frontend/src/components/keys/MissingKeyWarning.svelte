<script lang="ts">
    import { Modal } from "flowbite-svelte";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { checkLocalKeyAvailability } from "../../data/crypto/keyManager";
    import BackupRecover from "./backup/BackupRecover.svelte";
    import type { APIUserKey } from "@paltiverse/palform-typescript-openapi";

    export let missingKey: APIUserKey | undefined = undefined;
    export let showModal = false;
    const orgCtx = getOrgContext();

    (async () => {
        missingKey = await checkLocalKeyAvailability($orgCtx.myKeys);
        if (missingKey) showModal = true;
    })();
</script>

<Modal bind:open={showModal} outsideclose title="No keys found on this device">
    {#if missingKey}
        <p>You won't be able to decrypt any form responses.</p>
        <p>
            Don't worry! Simply enter your passphrase, and we'll download the
            key in a few seconds.
        </p>

        <BackupRecover
            key={missingKey}
            showInfo={false}
            disableLink
            on:successfulImport={() => (showModal = false)}
        />
    {/if}
</Modal>
