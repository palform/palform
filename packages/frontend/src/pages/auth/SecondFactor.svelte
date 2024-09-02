<script lang="ts">
    import { Input, Label } from "flowbite-svelte";
    import InfoText from "../../components/type/InfoText.svelte";
    import AuthCard from "../../layouts/AuthCard.svelte";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import { APIs } from "../../data/common";
    import Captcha from "../../components/captcha/Captcha.svelte";
    import { showFailureToast } from "../../data/toast";
    import { navigate } from "svelte-routing";
    import { saveAuthToken } from "../../data/auth";

    export let sessionId: string;

    const orgId = new URLSearchParams(location.search).get("thenOrg");

    let token = "";
    let captchaValue = "";
    let loading = false;
    $: onSubmit = async (e: Event) => {
        e.preventDefault();

        if (!captchaValue) {
            await showFailureToast("Please complete the captcha");
            return;
        }

        loading = true;
        try {
            const resp = await APIs.auth.authSignIn(captchaValue, {
                SecondFactor: {
                    session_id: sessionId,
                    token,
                },
            });

            if ("Done" in resp.data) {
                await saveAuthToken(resp.data.Done.token);
            } else {
                await showFailureToast("Unexpected response from server");
                return;
            }

            if (orgId) {
                navigate(`/orgs/${orgId}/induction/billing`);
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
        You're using two factor authentication to secure your account. To
        continue, please enter the code from your authenticator app.
    </InfoText>

    <form on:submit={onSubmit}>
        <Label class="mt-4">
            Code
            <Input
                bind:value={token}
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

    <svelte:fragment slot="footer">
        <InfoText>Can't get a code? Please contact our support team.</InfoText>
    </svelte:fragment>
</AuthCard>
