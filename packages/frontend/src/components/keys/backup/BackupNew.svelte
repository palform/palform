<script lang="ts">
    import { Checkbox, Helper, Input, Label } from "flowbite-svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import LoadingButton from "../../LoadingButton.svelte";
    import { backupKey } from "../../../data/crypto/keyManager";
    import { showSuccessToast } from "../../../data/toast";
    import { createEventDispatcher } from "svelte";
    import InfoText from "../../type/InfoText.svelte";
    import { generate_passphrase_js } from "@paltiverse/palform-crypto";

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

<Label class="mt-4 text-lg">
    Your key password
    <Input class="mt-2" size="lg" readonly value={recoveryPhrase} />
    <Helper class="mt-2"></Helper>
    <Helper class="font-medium"></Helper>
</Label>

<InfoText class="font-medium mt-4">
    You could lose your form responses if this password gets lost!
</InfoText>

<InfoText class="mt-2">
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
