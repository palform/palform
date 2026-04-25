<script lang="ts">
    import { faKey } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, Input, Label, Modal } from "flowbite-svelte";
    import type { APIAdminUserSecondAuthenticationFactor } from "@palform/palform-typescript-openapi";
    import { showSuccessToast } from "../../../../data/toast";
    import EnrollTotp from "./EnrollTOTP.svelte";
    import EnrollWebauthn from "./EnrollWebauthn.svelte";

    interface Props {
        onenroll: (factor: APIAdminUserSecondAuthenticationFactor) => void;
    }
    let { onenroll }: Props = $props();

    let showModal = $state(false);

    let step: "choose" | "totp" | "webauthn" = $state("choose");
    let nickname = $state("");
    let onContinueClick = $derived((nextStep: "totp" | "webauthn") => {
        if (!nickname.trim()) return;
        step = nextStep;
    });

    let onEnroll = $derived(
        async (e: APIAdminUserSecondAuthenticationFactor) => {
            await showSuccessToast("2FA method enrolled");
            onenroll(e);
            showModal = false;
        }
    );
</script>

<Button onclick={() => (showModal = true)}>
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
            <Button onclick={() => onContinueClick("totp")}>
                Authenticator app
            </Button>
            <Button onclick={() => onContinueClick("webauthn")}>Passkey</Button>
        {/if}
    {:else if step === "totp"}
        <EnrollTotp {nickname} onenroll={onEnroll} />
    {:else if step === "webauthn"}
        <EnrollWebauthn {nickname} onenroll={onEnroll} />
    {/if}
</Modal>
