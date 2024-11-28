<script lang="ts">
    import { Button, Input, Label } from "flowbite-svelte";
    import InfoText from "../../components/type/InfoText.svelte";
    import AuthCard from "../../layouts/AuthCard.svelte";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import { APIs } from "../../data/common";
    import Captcha from "../../components/captcha/Captcha.svelte";
    import { showFailureToast } from "../../data/toast";
    import { navigate } from "svelte-routing";
    import { saveAuthToken } from "../../data/auth";
    import type {
        SignInResponseOneOf1SecondFactorRequired,
        SignInSecondFactorRequest,
    } from "@paltiverse/palform-typescript-openapi";
    import WebauthnButton from "../../components/auth/WebauthnButton.svelte";
    import { FontAwesomeIcon } from "@fortawesome/svelte-fontawesome";
    import { faFingerprint, faMobile } from "@fortawesome/free-solid-svg-icons";

    export let tfa: SignInResponseOneOf1SecondFactorRequired;
    export let newOrgId: string | undefined;

    $: allowWebauthn = !!tfa.rcr;
    $: allowTotp = tfa.totp;
    $: allowBoth = allowWebauthn && allowTotp;
    let selectedMethod: "webauthn" | "totp" | undefined = undefined;

    let totpToken = "";
    let captchaValue = "";
    let loading = false;
    $: onTotpSubmit = async (e: Event) => {
        e.preventDefault();

        if (!captchaValue) {
            await showFailureToast("Please complete the captcha");
            return;
        }

        await submitWith({ Totp: totpToken });
    };

    $: onWebauthnAuth = async (e: CustomEvent<any>) => {
        await submitWith({ Webauthn: e.detail });
    };

    $: submitWith = async (data: SignInSecondFactorRequest) => {
        loading = true;
        try {
            const resp = await APIs.auth.authSignIn(captchaValue, {
                SecondFactor: {
                    session_id: tfa.session_id,
                    factor: data,
                },
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
    };
</script>

<AuthCard title="Verify your identity">
    <InfoText>
        You're using two factor authentication to secure your account.
    </InfoText>

    {#if allowBoth && selectedMethod === undefined}
        <div class="mt-4">
            <Button on:click={() => (selectedMethod = "totp")}>
                <FontAwesomeIcon icon={faMobile} class="me-2" />
                Authenticator app
            </Button>
            <Button on:click={() => (selectedMethod = "webauthn")}>
                <FontAwesomeIcon icon={faFingerprint} class="me-2" />
                Passkey
            </Button>
        </div>
    {:else if (!allowBoth && allowWebauthn) || selectedMethod === "webauthn"}
        {#if !loading}
            <Captcha
                class="mt-4"
                on:complete={(e) => (captchaValue = e.detail)}
                on:clear={() => (captchaValue = "")}
            />
        {/if}

        <WebauthnButton
            flowType="authenticate"
            class="mt-4"
            authCredential={tfa.rcr}
            on:authenticate={onWebauthnAuth}
            disabled={captchaValue === ""}
        />
    {:else if (!allowBoth && allowTotp) || selectedMethod === "totp"}
        <form on:submit={onTotpSubmit}>
            <Label class="mt-4">
                Code
                <Input
                    bind:value={totpToken}
                    class="mt-1"
                    disabled={loading}
                    required
                />
            </Label>

            {#if !loading}
                <Captcha
                    class="mt-4"
                    on:complete={(e) => (captchaValue = e.detail)}
                    on:clear={() => (captchaValue = "")}
                />
            {/if}

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

    <svelte:fragment slot="footer">
        <InfoText>Can't sign in? Please contact our support team.</InfoText>
    </svelte:fragment>
</AuthCard>
