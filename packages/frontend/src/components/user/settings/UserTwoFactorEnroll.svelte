<script lang="ts">
    import { faKey } from "@fortawesome/free-solid-svg-icons";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { Button, Input, Label, Modal } from "flowbite-svelte";
    import { generateNewTOTP, verifyTOTP } from "../../../data/auth/2fa";
    import LoadingButton from "../../LoadingButton.svelte";
    import { showFailureToast, showSuccessToast } from "../../../data/toast";
    import { APIs } from "../../../data/common";
    import { createEventDispatcher } from "svelte";
    import type { APIAdminUserSecondAuthenticationFactor } from "@paltiverse/palform-typescript-openapi";
    import { DateTime } from "luxon";
    import QrCode from "../../QRCode.svelte";

    const dispatch = createEventDispatcher<{
        enroll: APIAdminUserSecondAuthenticationFactor;
    }>();

    let showModal = false;

    let nickname = "";
    let data: { uri: string; secret: string } | null = null;

    $: onContinueClick = () => {
        if (!nickname.trim()) return;
        data = generateNewTOTP(nickname);
    };

    let sampleCode = "";
    let loading = false;
    $: onEnrollClick = async () => {
        if (!data) return;

        if (!verifyTOTP(sampleCode, data.secret)) {
            await showFailureToast("Invalid code provided. Please try again.");
            return;
        }

        loading = true;
        try {
            const resp = await APIs.secondFactors().then((a) =>
                a.userSecondFactorsEnroll({
                    nickname,
                    secret: data!.secret,
                })
            );

            dispatch("enroll", {
                id: resp.data,
                nickname,
                created_at: DateTime.now().toISO(),
            });
            await showSuccessToast("2FA method enrolled");
            loading = false;
            data = null;
            showModal = false;
        } catch (e) {
            await showFailureToast(e);
        }
    };
</script>

<Button on:click={() => (showModal = true)}>
    <FontAwesomeIcon icon={faKey} class="me-3" />
    Enroll new method
</Button>

<Modal bind:open={showModal} outsideclose title="Enroll 2FA method">
    {#if !data}
        <p>Currently only authenticator apps are supported.</p>
        <p>
            Please choose a nickname for your app (e.g. the name of the device
            you'll be storing the code on).
        </p>

        <Label>
            Nickname
            <Input
                bind:value={nickname}
                class="mt-1"
                on:click={onContinueClick}
            />
        </Label>

        <Button on:click={onContinueClick}>Continue</Button>
    {:else}
        <QrCode uri={data.uri} />
        <Label>
            URI
            <Input readonly value={data.uri} class="mt-1" disabled={loading} />
        </Label>

        <p>
            Please scan this code with your authenticator app or copy the URI
            above.
        </p>

        <p>When you're ready, enter the code shown in your app.</p>
        <Label>
            Code shown in app
            <Input class="mt-1" bind:value={sampleCode} disabled={loading} />
        </Label>

        <LoadingButton {loading} disabled={loading} on:click={onEnrollClick}>
            Enroll
        </LoadingButton>
    {/if}
</Modal>
