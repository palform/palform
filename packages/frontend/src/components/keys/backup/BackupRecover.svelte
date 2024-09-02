<script lang="ts">
    import { Helper, Input, Label } from "flowbite-svelte";

    import InfoText from "../../type/InfoText.svelte";
    import SectionHeading from "../../type/SectionHeading.svelte";
    import LoadingButton from "../../LoadingButton.svelte";
    import type { APIUserKey } from "@paltiverse/palform-typescript-openapi";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { restoreKeyFromBackup } from "../../../data/crypto/keyManager";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { navigate } from "svelte-routing";

    export let key: APIUserKey;
    const orgCtx = getOrgContext();

    let recoverLoading = false;
    let recoveryPhrase = "";
    $: onRecoverClick = async (e: Event) => {
        e.preventDefault();

        recoverLoading = true;
        try {
            await restoreKeyFromBackup($orgCtx.org.id, key.id, recoveryPhrase);
            await showSuccessToast(
                "Successfully restored from backup! You can now use this key to decrypt responses.",
            );
            navigate(`/orgs/${$orgCtx.org.id}/settings/keys`);
        } catch (e) {
            console.error(e);
            await showFailureToast(
                "Failed to decrypt backup. Please check your recovery passphrase and try again.",
            );
        }
        recoverLoading = false;
    };
</script>

<SectionHeading class="mt-4">Restore from backup</SectionHeading>
<InfoText>
    You don't seem to have this key stored in your browser; you won't be able to
    use it to decrypt form responses.
</InfoText>
<InfoText>
    If you have this key's recovery phrase, you can restore the backup and
    import the private key into your browser's storage.
</InfoText>

<form class="mt-4" on:submit={onRecoverClick}>
    <Label>
        Recovery phrase
        <Input
            class="mt-2"
            bind:value={recoveryPhrase}
            disabled={recoverLoading}
            required
        />
        <Helper class="mt-2">
            This is a series of 16 space-separated words, all lowercase.
        </Helper>
    </Label>

    <LoadingButton
        buttonClass="mt-4"
        type="submit"
        loading={recoverLoading}
        disabled={recoverLoading}
    >
        Recover
    </LoadingButton>
</form>
