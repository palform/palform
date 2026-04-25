<script lang="ts">
    import { Button, Checkbox } from "flowbite-svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import LoadingButton from "../../LoadingButton.svelte";
    import { backupKey } from "../../../data/crypto/keyManager";
    import { showSuccessToast } from "../../../data/toast";
    import InfoText from "../../type/InfoText.svelte";
    import { generate_passphrase_js } from "@palform/palform-crypto";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import {
        faCopy,
        faDownload,
        faRotateRight,
    } from "@fortawesome/free-solid-svg-icons";
    import { copyGenericValue } from "../../../data/util/clipboard";

    interface Props {
        keyId: string;
        showInfo?: boolean;
        ondone: () => void;
    }

    let { keyId, showInfo = true, ondone }: Props = $props();
    const orgCtx = getOrgContext();

    let recoveryWords = $state(generate_passphrase_js());
    let recoveryPhrase = $derived(recoveryWords.join(" "));

    let phraseWrittenDown = $state(false);
    let backupLoading = $state(false);
    let onBackupClick = $derived(async () => {
        if (recoveryPhrase === "") return;
        backupLoading = true;
        await backupKey($orgCtx.org.id, keyId, recoveryPhrase);
        backupLoading = false;
        await showSuccessToast("Key backed up to Palform server");
        ondone();
    });

    const onRegenerate = () => {
        recoveryWords = generate_passphrase_js();
    };

    let onCopyClick = $derived(async () => {
        if (recoveryPhrase === "") return;
        await copyGenericValue(recoveryPhrase);
        phraseWrittenDown = true;
    });

    let onDownloadClick = $derived(async () => {
        if (recoveryPhrase === "") return;
        const el = document.createElement("a");
        el.setAttribute(
            "href",
            `data:text/plain;charset=utf-8,${encodeURIComponent(recoveryPhrase)}`
        );
        el.setAttribute("download", "palform_key_recovery_phrase.txt");
        el.click();

        phraseWrittenDown = true;
    });
</script>

<div class="flex gap-4 flex-wrap">
    {#each recoveryWords as word, index}
        <p
            class="bg-primary-100 dark:bg-slate-600 text-primary-950 dark:text-white px-3 py-1 text-center text-lg rounded-lg"
        >
            <span class="text-primary-600 dark:text-gray-400">{index + 1}</span>
            {word}
        </p>
    {/each}
</div>

<div class="mt-8 mb-8">
    <Button color="light" onclick={onCopyClick} disabled={backupLoading}>
        <FontAwesomeIcon icon={faCopy} class="me-2" />
        Copy
    </Button>
    <Button color="light" onclick={onDownloadClick} disabled={backupLoading}>
        <FontAwesomeIcon icon={faDownload} class="me-2" />
        Download
    </Button>
    <Button color="light" onclick={onRegenerate} disabled={backupLoading}>
        <FontAwesomeIcon icon={faRotateRight} class="me-2" />
        Generate new words
    </Button>
</div>

{#if showInfo}
    <InfoText class="font-medium mt-4">
        This is your recovery password. Make sure to keep it safe!
    </InfoText>

    <InfoText class="font-medium mt-2">
        You could lose your form responses if this password gets lost.
    </InfoText>

    <InfoText class="mt-2">
        We recommend saving this in your password manager, or alternatively
        printing it out or writing it down.
    </InfoText>
{/if}

<div class="mt-4">
    <Checkbox bind:checked={phraseWrittenDown} disabled={backupLoading}>
        I have saved my words somewhere secure
    </Checkbox>
</div>

<LoadingButton
    buttonClass="mt-4"
    disabled={backupLoading || !phraseWrittenDown}
    loading={backupLoading}
    onclick={onBackupClick}
    size="lg"
>
    Finish key setup
</LoadingButton>
