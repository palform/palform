<script lang="ts">
    import { Checkbox, Helper, Input, Label } from "flowbite-svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { generate_passphrase_js } from "@paltiverse/palform-client-common";
    import LoadingButton from "../../LoadingButton.svelte";
    import { backupKey } from "../../../data/crypto/keyManager";
    import { showSuccessToast } from "../../../data/toast";
    import { createEventDispatcher } from "svelte";
    import InfoText from "../../type/InfoText.svelte";

    export let keyId: string;
    const orgCtx = getOrgContext();
    const dispatch = createEventDispatcher<{ done: undefined }>();

    let recoveryPhrase = generate_passphrase_js().join(" ");

    let phraseWrittenDown = false;
    let backupLoading = false;
    $: onBackupClick = async () => {
        if (recoveryPhrase === "") return;
        backupLoading = true;
        await backupKey($orgCtx.org.id, keyId, recoveryPhrase);
        backupLoading = false;
        await showSuccessToast("Key backed up to Palform server");
        dispatch("done");
    };
</script>

<InfoText>
    You'll need this password whenever you want to add Palform to a new device
    or if the key accidentally gets erased from your browser.
</InfoText>
<InfoText class="font-medium">
    You could lose your form responses if this password gets lost!
</InfoText>

<Label class="mt-4">
    Your key password
    <Input class="mt-2" readonly value={recoveryPhrase} />
    <Helper class="mt-2"></Helper>
    <Helper class="font-medium"></Helper>
</Label>

<InfoText class="mt-4">
    We recommend saving this in your password manager, or alternatively printing
    it out or writing it down.
</InfoText>

<Checkbox
    bind:checked={phraseWrittenDown}
    class="mt-4"
    disabled={backupLoading}
>
    I have saved my password somewhere secure
</Checkbox>

<LoadingButton
    buttonClass="mt-4"
    disabled={backupLoading || !phraseWrittenDown}
    loading={backupLoading}
    on:click={onBackupClick}
>
    Finish key setup
</LoadingButton>
