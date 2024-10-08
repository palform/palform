<script lang="ts">
    import { Button, ButtonGroup, Fileupload, Label } from "flowbite-svelte";
    import LoadingButton from "../../../components/LoadingButton.svelte";
    import {
        getOrgContext,
        reloadGlobalAlert,
        reloadInduction,
    } from "../../../data/contexts/orgLayout";
    import {
        showFailureToast,
        showSuccessToast,
        showToast,
    } from "../../../data/toast";
    import { faWarning } from "@fortawesome/free-solid-svg-icons";
    import { importKey } from "../../../data/crypto/keyManager";
    import { navigate } from "svelte-routing";
    import InfoText from "../../../components/type/InfoText.svelte";
    import MainTitle from "../../../layouts/MainTitle.svelte";
    import { navigateEvent } from "@paltiverse/palform-frontend-common";

    const orgCtx = getOrgContext();

    let loading = false;
    let keyFiles: FileList;

    $: onSubmit = async (e: Event) => {
        e.preventDefault();

        if (keyFiles.length !== 1) {
            await showToast({
                label: "Please select exactly one file",
                color: "orange",
                icon: faWarning,
            });
            return;
        }

        loading = true;
        try {
            const keyText = await keyFiles[0].text();
            const serverId = await importKey(keyText, $orgCtx.org.id);
            await reloadGlobalAlert(orgCtx);
            await reloadInduction(orgCtx);
            await showSuccessToast("Key imported successfully");
            navigate(
                `/orgs/${$orgCtx.org.id}/user/keys/${serverId}/backup?isNew=y`
            );
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
</script>

<MainTitle>Import a key</MainTitle>

<InfoText class="mt-2">
    Import an OpenPGP certificate (including the secret) to use for encrypting
    form responses.
</InfoText>
<InfoText class="mb-4">
    Simply paste your ASCII-armored certificate in the field below and we'll
    import it.
</InfoText>

<form on:submit={onSubmit}>
    <fieldset>
        <Label>
            OpenPGP ASCII-armored certificate
            <Fileupload
                class="mt-2"
                bind:files={keyFiles}
                accept=".asc,.pgp,.key,application/pgp-keys"
            />
        </Label>
    </fieldset>

    <ButtonGroup class="mt-4">
        <LoadingButton type="submit" disabled={loading} {loading}>
            Import
        </LoadingButton>
        <Button
            color="primary"
            href={`/orgs/${$orgCtx.org.id}/user/keys`}
            on:click={navigateEvent}
            outline
        >
            Cancel
        </Button>
    </ButtonGroup>
</form>
