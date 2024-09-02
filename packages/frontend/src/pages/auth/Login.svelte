<script lang="ts">
    import AuthCard from "../../layouts/AuthCard.svelte";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import {
        getOrgSubdomain,
        getSignInURL,
        signInWithEmailAndPassword,
    } from "../../data/auth";
    import InfoText from "../../components/type/InfoText.svelte";
    import { showFailureToast } from "../../data/toast";
    import { Helper, Input, Label } from "flowbite-svelte";
    import { navigate } from "svelte-routing";
    import Captcha from "../../components/captcha/Captcha.svelte";
    import TextButton from "../../components/TextButton.svelte";

    let loading = false;
    const isOrgLogin = getOrgSubdomain();
    const onOrgSignInClick = async () => {
        loading = true;

        try {
            const url = await getSignInURL();
            if (!url) {
                await showFailureToast(
                    "Could not find organisation to sign into. Please check you are on the correct subdomain."
                );
                loading = false;
                return;
            }
            window.location.href = url;
        } catch (e) {
            await showFailureToast(e);
            loading = false;
        }
    };

    let email = "";
    let password = "";
    let captcha = "";
    const createInitialOrg = new URLSearchParams(window.location.search).has(
        "create_initial_org"
    );

    $: onSignInClick = async (e: Event) => {
        e.preventDefault();
        if (!email || !password) return;
        if (!captcha) {
            await showFailureToast("Please complete the captcha");
            return;
        }

        loading = true;
        try {
            const { newOrgId, tfaSessionId } = await signInWithEmailAndPassword(
                email,
                password,
                captcha,
                createInitialOrg
            );

            if (tfaSessionId) {
                navigate(
                    `/auth/login/tfa/${tfaSessionId}${newOrgId ? "?thenOrg=" + newOrgId : ""}`
                );
            } else {
                if (newOrgId) {
                    navigate(`/orgs/${newOrgId}/induction/billing`);
                } else {
                    navigate("/");
                }
            }
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
</script>

{#if isOrgLogin}
    <AuthCard title="Log in to Palform">
        <InfoText class="mb-4">
            Sign in with your organisation to continue.
        </InfoText>
        <LoadingButton {loading} disabled={loading} on:click={onOrgSignInClick}>
            Sign in
        </LoadingButton>
    </AuthCard>
{:else}
    <AuthCard title="Log in to Palform">
        <form class="space-y-6" on:submit={onSignInClick}>
            <Label>
                Email address
                <Input
                    class="mt-2"
                    type="email"
                    required
                    disabled={loading}
                    bind:value={email}
                />
            </Label>
            <Label>
                Password
                <Input
                    class="mt-2"
                    type="password"
                    required
                    disabled={loading}
                    bind:value={password}
                />
                <Helper class="mt-2">
                    <TextButton class="!text-xs" href="/auth/reset/password">
                        I forgot my password
                    </TextButton>
                </Helper>
            </Label>

            {#if !loading}
                <Captcha
                    on:complete={(e) => (captcha = e.detail)}
                    on:clear={() => (captcha = "")}
                />
            {/if}

            <LoadingButton {loading} disabled={loading} type="submit">
                Sign in
            </LoadingButton>
        </form>

        <svelte:fragment slot="footer">
            <InfoText>
                Need an account?
                <TextButton class="inline-block" href="/auth/signup"
                    >Sign up for free</TextButton
                >
            </InfoText>
        </svelte:fragment>
    </AuthCard>
{/if}
