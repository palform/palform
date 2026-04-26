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
    import { onMount } from "svelte";

    interface Props {
        flowType: "register" | "authenticate";
        registerNickname?: string;
        authCredential?: any | undefined;
        forceLoading?: boolean;
        disabled?: boolean;
        initialAutoClick?: boolean;
        class?: string;
        onenroll?: (data: string) => void;
        onauthenticate?: (data: any) => void;
    }

    let {
        flowType,
        registerNickname = undefined,
        authCredential = undefined,
        forceLoading = false,
        disabled = false,
        initialAutoClick = false,
        class: className,
        onenroll,
        onauthenticate,
    }: Props = $props();

    let loading = $state(false);
    async function onButtonClick() {
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

                onenroll?.(enrollResp.data);
            } else if (flowType === "authenticate") {
                if (!authCredential) return;

                const result = await startAuthentication({
                    optionsJSON: authCredential.publicKey,
                });
                onauthenticate?.(result);
            }
        } catch (_) {
            await showFailureToast(
                "Failed to authenticate passkey. Please try again."
            );
        } finally {
            loading = false;
        }
    }

    onMount(() => {
        if (initialAutoClick) void onButtonClick();
    });
</script>

<LoadingButton
    onclick={onButtonClick}
    loading={loading || forceLoading}
    disabled={loading || forceLoading || disabled}
    buttonClass={className}
>
    <FontAwesomeIcon icon={faFingerprint} class="me-2" />
    {#if flowType === "register"}
        Register passkey
    {:else}
        Continue with passkey
    {/if}
</LoadingButton>
