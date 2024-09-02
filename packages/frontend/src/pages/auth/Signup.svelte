<script lang="ts">
    import { Checkbox, Helper, Input, Label } from "flowbite-svelte";
    import AuthCard from "../../layouts/AuthCard.svelte";
    import InfoText from "../../components/type/InfoText.svelte";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import { APIs } from "../../data/common";
    import { showFailureToast } from "../../data/toast";
    import Captcha from "../../components/captcha/Captcha.svelte";
    import TextButton from "../../components/TextButton.svelte";
    import PasswordPicker from "../../components/password/PasswordPicker.svelte";

    let displayName = "";
    let email = "";
    let password = "";
    let captcha = "";

    let loading = false;
    let signUpComplete = false;
    $: onSignUpClick = async (e: Event) => {
        e.preventDefault();

        if (!captcha) {
            await showFailureToast("Please complete the captcha");
            return;
        }

        loading = true;
        try {
            await APIs.auth.authSignUp(captcha, {
                display_name: displayName,
                email,
                password,
            });
            signUpComplete = true;
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
</script>

{#if signUpComplete}
    <AuthCard title="Verify your email">
        <InfoText>Thanks for signing up!</InfoText>
        <InfoText>
            We've sent a message to <strong>{email}</strong> with a link. Please
            click it within the next 15 minutes to verify your email address.
        </InfoText>
    </AuthCard>
{:else}
    <AuthCard title="Sign up for Palform">
        <InfoText>All fields are required</InfoText>

        <form class="mt-4 space-y-6" on:submit={onSignUpClick}>
            <Label>
                Your name
                <Input
                    class="mt-2"
                    required
                    bind:value={displayName}
                    disabled={loading}
                />
                <Helper class="mt-2">
                    We'll show this name in your organisations to help your
                    coworkers find you. It can be different from your legal name
                    for billing purposes.
                </Helper>
            </Label>

            <Label>
                Your email address
                <Input
                    class="mt-2"
                    type="email"
                    required
                    bind:value={email}
                    disabled={loading}
                />
            </Label>

            <Label>
                Password
                <PasswordPicker
                    bind:value={password}
                    class="mt-2"
                    required
                    disabled={loading}
                />
                <Helper class="mt-2">
                    We'll set up your account with a password for now. If you
                    need to, you can set up an organisation with OpenID Connect
                    later.
                </Helper>
            </Label>

            {#if !loading}
                <Captcha
                    on:complete={(e) => (captcha = e.detail)}
                    on:clear={() => (captcha = "")}
                />
            {/if}

            <Checkbox required>
                I agree with the&nbsp;<TextButton
                    href="https://palform.app/legal/terms"
                    >Terms of Use</TextButton
                >&nbsp;and&nbsp;<TextButton
                    href="https://palform.app/legal/privacy"
                    >Privacy Policy</TextButton
                >.
            </Checkbox>

            <LoadingButton type="submit" disabled={loading} {loading}>
                Sign up
            </LoadingButton>
        </form>

        <svelte:fragment slot="footer">
            <InfoText>
                Already signed up?
                <TextButton class="inline-block" href="/auth/login"
                    >Log in</TextButton
                >
            </InfoText>
        </svelte:fragment>
    </AuthCard>
{/if}
