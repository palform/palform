<script lang="ts">
    import { Input, Label } from "flowbite-svelte";
    import QrCode from "../../../QRCode.svelte";
    import LoadingButton from "../../../LoadingButton.svelte";
    import { generateNewTOTP, verifyTOTP } from "../../../../data/auth/2fa";
    import { showFailureToast } from "../../../../data/toast";
    import { APIs } from "../../../../data/common";
    import { createEventDispatcher } from "svelte";
    import type {
        APIAdminUserSecondAuthenticationFactor,
    } from "@paltiverse/palform-typescript-openapi";
    import { DateTime } from "luxon";

    export let nickname: string;
    const dispatch = createEventDispatcher<{
        enroll: APIAdminUserSecondAuthenticationFactor;
    }>();

    let data: { uri: string; secret: string } = generateNewTOTP(nickname);
    let loading = false;
    let sampleCode = "";

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
            loading = false;
        } catch (e) {
            await showFailureToast(e);
        }
    };
</script>

<QrCode uri={data.uri} />
<Label>
    URI
    <Input readonly value={data.uri} class="mt-1" disabled={loading} />
</Label>

<p>Please scan this code with your authenticator app or copy the URI above.</p>

<p>When you're ready, enter the code shown in your app.</p>
<Label>
    Code shown in app
    <Input class="mt-1" bind:value={sampleCode} disabled={loading} />
</Label>

<LoadingButton {loading} disabled={loading} on:click={onEnrollClick}>
    Enroll
</LoadingButton>
