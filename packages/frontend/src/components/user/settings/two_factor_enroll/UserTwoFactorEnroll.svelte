<script lang="ts">
    import { faKey } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, Input, Label, Modal } from "flowbite-svelte";
    import { createEventDispatcher } from "svelte";
    import type { APIAdminUserSecondAuthenticationFactor } from "@paltiverse/palform-typescript-openapi";
    import { showSuccessToast } from "../../../../data/toast";
    import EnrollTotp from "./EnrollTOTP.svelte";
    import EnrollWebauthn from "./EnrollWebauthn.svelte";

    const dispatch = createEventDispatcher<{
        enroll: APIAdminUserSecondAuthenticationFactor;
    }>();

    let showModal = false;

    let step: "choose" | "totp" | "webauthn" = "choose";
    let nickname = "";
    $: onContinueClick = (nextStep: "totp" | "webauthn") => {
        if (!nickname.trim()) return;
        step = nextStep;
    };

    $: onEnroll = async (
        e: CustomEvent<APIAdminUserSecondAuthenticationFactor>
    ) => {
        await showSuccessToast("2FA method enrolled");
        dispatch("enroll", e.detail);
        showModal = false;
    };
</script>

<Button on:click={() => (showModal = true)}>
    <FontAwesomeIcon icon={faKey} class="me-3" />
    Enroll new method
</Button>

<Modal bind:open={showModal} outsideclose title="Enroll 2FA method">
    {#if step === "choose"}
        <p>
            Please choose a nickname for your 2FA method (e.g. the name of the
            device you'll be storing it on).
        </p>

        <Label>
            Nickname
            <Input bind:value={nickname} class="mt-1" />
        </Label>

        {#if nickname.trim().length !== 0}
            <p>Now, choose which method you'd like to enroll.</p>
            <Button on:click={() => onContinueClick("totp")}>
                Authenticator app
            </Button>
            <Button on:click={() => onContinueClick("webauthn")}>Passkey</Button
            >
        {/if}
    {:else if step === "totp"}
        <EnrollTotp {nickname} on:enroll={onEnroll} />
    {:else if step === "webauthn"}
        <EnrollWebauthn {nickname} on:enroll={onEnroll} />
    {/if}
</Modal>
