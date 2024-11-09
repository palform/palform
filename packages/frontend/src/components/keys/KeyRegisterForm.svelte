<script lang="ts">
    import { faExclamationTriangle } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";

    import {
        Accordion,
        AccordionItem,
        Alert,
        Label,
        Select,
    } from "flowbite-svelte";
    import LoadingButton from "../LoadingButton.svelte";
    import {
        getOrgContext,
        reloadGlobalAlert,
        reloadInduction,
    } from "../../data/contexts/orgLayout";
    import { showFailureToast, showSuccessToast } from "../../data/toast";
    import { registerKey } from "../../data/crypto/keyManager";
    import { createEventDispatcher, onMount } from "svelte";
    import BackupNew from "./backup/BackupNew.svelte";
    import KeyPersistenceModal from "./KeyPersistenceModal.svelte";
    import { UAParser } from "ua-parser-js";

    export let showInfo = true;
    export let autoCreate = false;

    const ctx = getOrgContext();
    const browserSupport = !!(navigator.storage && navigator.storage.persist);
    const dispatch = createEventDispatcher<{ done: string }>();

    const browserAgent = UAParser();

    let expirationDays = 0;
    let registerLoading = false;
    let backupKeyId: string | undefined;
    let showPersistenceModal = false;
    $: onRegister = async () => {
        const granted = await navigator.storage.persisted();
        if (!granted) {
            if (browserAgent.browser.name !== "Firefox") {
                await navigator.storage.persist();
            } else {
                showPersistenceModal = true;
                return;
            }
        } else {
            showPersistenceModal = false;
        }

        registerLoading = true;
        try {
            const id = await registerKey(expirationDays, $ctx.org.id);
            if (!id) return;
            await reloadGlobalAlert(ctx);
            await reloadInduction(ctx);

            if (!autoCreate) {
                await showSuccessToast(
                    "Key created successfully! Make sure to create a backup."
                );
            }

            backupKeyId = id;
        } catch (e) {
            showFailureToast(e);
        }

        registerLoading = false;
    };

    $: onBackupCreateDone = () => {
        if (!backupKeyId) return;
        dispatch("done", backupKeyId);
    };

    onMount(() => {
        if (autoCreate) {
            onRegister();
        }
    });
</script>

<KeyPersistenceModal bind:open={showPersistenceModal} on:granted={onRegister} />

{#if backupKeyId}
    <BackupNew {showInfo} keyId={backupKeyId} on:done={onBackupCreateDone} />
{:else}
    {#if !browserSupport}
        <Alert color="red" border>
            <FontAwesomeIcon slot="icon" icon={faExclamationTriangle} />
            <p>
                <strong>Warning!</strong>
                It looks like your browser doesn't support long-term key storage.
            </p>
            <p>
                You can still create a key if you'd like, but make sure to back
                it up as your browser might not save it properly.
            </p>
        </Alert>
    {/if}

    {#if showInfo}
        <Alert class="mt-2 space-y-2" border>
            <p>
                This secret key will be used to encrypt form responses to make
                sure nobody but you can see them.
            </p>
            <p>
                We'll store a copy in your browser and one on our server,
                encrypted with a password that you'll see on the next page.
            </p>
            {#if expirationDays !== 0}
                <p>
                    When your key expires, you can continue using it to decrypt
                    existing responses, but you'll need a new key to accept new
                    responses. We'll email you in advance of your expiration
                    date(s) to remind you.
                </p>
            {/if}
        </Alert>
    {/if}

    {#if !autoCreate}
        <form class="mt-4">
            <LoadingButton
                loading={registerLoading}
                disabled={registerLoading}
                on:click={onRegister}
                buttonClass="mt-4"
            >
                Generate my key
            </LoadingButton>

            <Accordion flush class="mt-2">
                <AccordionItem>
                    <span slot="header" class="text-sm">Advanced</span>
                    <fieldset>
                        <Label for="expiry" class="mb-2">Expires</Label>
                        <Select
                            id="expiry"
                            disabled={registerLoading}
                            items={[
                                { value: 30, name: "in 1 month" },
                                { value: 60, name: "in 2 months" },
                                { value: 180, name: "in 6 months" },
                                { value: 365, name: "in 1 year" },
                                { value: 730, name: "in 2 years" },
                                { value: 1825, name: "in 5 years" },
                                { value: 0, name: "Never" },
                            ]}
                            bind:value={expirationDays}
                        />
                    </fieldset>
                </AccordionItem>
            </Accordion>
        </form>
    {/if}
{/if}
