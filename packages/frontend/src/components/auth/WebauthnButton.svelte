<script lang="ts">
    import { APIs } from "../../data/common";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faFingerprint } from "@fortawesome/free-solid-svg-icons";
    import {
        startAuthentication,
        startRegistration,
    } from "@simplewebauthn/browser";
    import { showFailureToast } from "../../data/toast";
    import LoadingButton from "../LoadingButton.svelte";
    import { createEventDispatcher } from "svelte";

    export let flowType: "register" | "authenticate";
    export let registerNickname: string | undefined = undefined;
    export let authCredential: any | undefined = undefined;
    export let forceLoading = false;
    export let disabled = false

    const dispatch = createEventDispatcher<{
        enroll: string;
        authenticate: any;
    }>();

    let loading = false;
    $: onButtonClick = async () => {
        loading = true;

        try {
            if (flowType === "register") {
                if (!registerNickname) return;

                const resp = await APIs.secondFactors().then((a) =>
                    a.userSecondFactorsStartWebauthn()
                );

                const result = await startRegistration({
                    optionsJSON: (resp.data.ccr as any).publicKey,
                });

                const enrollResp = await APIs.secondFactors().then((a) =>
                    a.userSecondFactorsEnrollWebauthn({
                        cred: result as unknown as string,
                        session: resp.data.session,
                        nickname: registerNickname,
                    })
                );

                dispatch("enroll", enrollResp.data);
            } else if (flowType === "authenticate") {
                if (!authCredential) return;

                const result = await startAuthentication({
                    optionsJSON: authCredential.publicKey,
                });
                dispatch("authenticate", result);
            }
        } catch (_) {
            await showFailureToast(
                "Failed to authenticate passkey. Please try again."
            );
        } finally {
            loading = false;
        }
    };
</script>

<LoadingButton
    on:click={onButtonClick}
    loading={loading || forceLoading}
    disabled={loading || forceLoading || disabled}
    buttonClass={$$props.class}
>
    <FontAwesomeIcon icon={faFingerprint} class="me-2" />
    {#if flowType === "register"}
        Register passkey
    {:else}
        Continue with passkey
    {/if}
</LoadingButton>
