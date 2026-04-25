<script lang="ts">
    import { Input, Label } from "flowbite-svelte";
    import QrCode from "../../../QRCode.svelte";
    import LoadingButton from "../../../LoadingButton.svelte";
    import { generateNewTOTP, verifyTOTP } from "../../../../data/auth/2fa";
    import { showFailureToast } from "../../../../data/toast";
    import { APIs } from "../../../../data/common";
    import type { APIAdminUserSecondAuthenticationFactor } from "@palform/palform-typescript-openapi";
    import { DateTime } from "luxon";

    interface Props {
        nickname: string;
        onenroll: (factor: APIAdminUserSecondAuthenticationFactor) => void;
    }

    let { nickname, onenroll }: Props = $props();

    let data: { uri: string; secret: string } = $derived(
        generateNewTOTP(nickname)
    );
    let loading = $state(false);
    let sampleCode = $state("");

    let onEnrollClick = $derived(async () => {
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

            onenroll({
                id: resp.data,
                nickname,
                created_at: DateTime.now().toISO(),
                method: "TOTP",
            });
            loading = false;
        } catch (e) {
            await showFailureToast(e);
        }
    });
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

<LoadingButton {loading} disabled={loading} onclick={onEnrollClick}>
    Enroll
</LoadingButton>
