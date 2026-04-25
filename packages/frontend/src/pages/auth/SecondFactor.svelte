<script lang="ts">
    import { Button, Input, Label } from "flowbite-svelte";
    import InfoText from "../../components/type/InfoText.svelte";
    import AuthCard from "../../layouts/AuthCard.svelte";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import { APIs } from "../../data/common";
    import { showFailureToast } from "../../data/toast";
    import { navigate } from "../../router";
    import { saveAuthToken } from "../../data/auth";
    import type {
        SecondFactorRequiredSecondFactorRequired,
        VerifyTFASecondFactorRequest,
    } from "@palform/palform-typescript-openapi";
    import WebauthnButton from "../../components/auth/WebauthnButton.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faFingerprint, faMobile } from "@fortawesome/free-solid-svg-icons";

    interface Props {
        tfa: SecondFactorRequiredSecondFactorRequired;
        newOrgId: string | undefined;
    }

    let { tfa, newOrgId }: Props = $props();

    let selectedMethod: "webauthn" | "totp" | undefined = $state(undefined);

    let totpToken = $state("");
    let loading = $state(false);

    let allowWebauthn = $derived(!!tfa.rcr);
    let allowTotp = $derived(tfa.totp);
    let allowBoth = $derived(allowWebauthn && allowTotp);
    let submitWith = $derived(async (data: VerifyTFASecondFactorRequest) => {
        loading = true;
        try {
            const resp = await APIs.auth.authVerifyTfa({
                session_id: tfa.session_id,
                factor: data,
            });

            if ("Done" in resp.data) {
                await saveAuthToken(resp.data.Done.token);
            } else {
                await showFailureToast("Unexpected response from server");
                return;
            }

            if (newOrgId) {
                navigate(`/orgs/${newOrgId}/induction/billing`);
            } else {
                navigate("/");
            }
        } catch (e) {
            await showFailureToast(e);
        }

        loading = false;
    });
    let onTotpSubmit = $derived(async (e: Event) => {
        e.preventDefault();
        await submitWith({ Totp: totpToken });
    });
    let onWebauthnAuth = $derived(async (e: CustomEvent<any>) => {
        await submitWith({ Webauthn: e.detail });
    });
</script>

<AuthCard title="Verify your identity">
    <InfoText>
        You're using two factor authentication to secure your account.
    </InfoText>

    {#if allowBoth && selectedMethod === undefined}
        <div class="mt-4">
            <Button onclick={() => (selectedMethod = "totp")}>
                <FontAwesomeIcon icon={faMobile} class="me-2" />
                Authenticator app
            </Button>
            <Button onclick={() => (selectedMethod = "webauthn")}>
                <FontAwesomeIcon icon={faFingerprint} class="me-2" />
                Passkey
            </Button>
        </div>
    {:else if (!allowBoth && allowWebauthn) || selectedMethod === "webauthn"}
        <WebauthnButton
            flowType="authenticate"
            class="mt-4"
            authCredential={tfa.rcr}
            on:authenticate={onWebauthnAuth}
            initialAutoClick
        />
    {:else if (!allowBoth && allowTotp) || selectedMethod === "totp"}
        <form onsubmit={onTotpSubmit}>
            <Label class="mt-4">
                Code
                <Input
                    bind:value={totpToken}
                    class="mt-1"
                    disabled={loading}
                    required
                />
            </Label>

            <LoadingButton
                buttonClass="mt-3"
                type="submit"
                disabled={loading}
                {loading}
            >
                Continue
            </LoadingButton>
        </form>
    {/if}

    {#snippet footer()}
        <InfoText>Can't sign in? Please contact our support team.</InfoText>
    {/snippet}
</AuthCard>
