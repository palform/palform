<script lang="ts">
    import { Alert, Button, Fileupload, Label } from "flowbite-svelte";
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
    import { navigate, p } from "../../../router";
    import InfoText from "../../../components/type/InfoText.svelte";
    import MainTitle from "../../../layouts/MainTitle.svelte";
    import { isEntitled } from "../../../data/billing/entitlement";

    const orgCtx = getOrgContext();

    let loading = $state(false);
    let keyFiles: FileList | undefined = $state();
    let entitled = isEntitled("import_keys");

    let onSubmit = $derived(async (e: Event) => {
        e.preventDefault();
        if (!keyFiles) return;

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
    });
</script>

<MainTitle>Import a key</MainTitle>

{#if $entitled}
    <InfoText class="mt-2">
        Import an OpenPGP certificate (including the secret) to use for
        encrypting form responses.
    </InfoText>
    <InfoText class="mb-4">
        Simply paste your ASCII-armored certificate in the field below and we'll
        import it.
    </InfoText>

    <form onsubmit={onSubmit}>
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

        <div class="mt-4">
            <LoadingButton type="submit" disabled={loading} {loading}>
                Import
            </LoadingButton>
            <Button
                color="primary"
                href={p("/orgs/:orgId/user/keys", {
                    params: { orgId: $orgCtx.org.id },
                })}
                outline
            >
                Cancel
            </Button>
        </div>
    </form>
{:else}
    <Alert border class="mt-4">
        <h3 class="text-lg">Import your own OpenPGP certificates</h3>
        <p>
            Customise security to your own exact needs by bringing your own
            OpenPGP certificate. Your secret will be stored securely in your
            browser and will be sent to our server fully encrypted.
        </p>
        <p>To continue, please upgrade your plan.</p>

        <Button class="mt-2" href={`/orgs/${$orgCtx.org.id}/settings/billing`}
            >Continue</Button
        >
    </Alert>
{/if}
