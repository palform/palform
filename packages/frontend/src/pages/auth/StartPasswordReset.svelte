<script lang="ts">
    import { Input, Label } from "flowbite-svelte";
    import InfoText from "../../components/type/InfoText.svelte";
    import AuthCard from "../../layouts/AuthCard.svelte";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import { APIs } from "../../data/common";
    import { showFailureToast } from "../../data/toast";

    let email = "";
    let loading = false;
    let sent = false;

    $: onSubmit = async (e: Event) => {
        e.preventDefault();

        loading = true;
        try {
            await APIs.passwordReset.userPasswordResetSend({
                email,
            });
            sent = true;
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
</script>

<AuthCard title="Reset password">
    {#if sent}
        <InfoText>
            We've sent you an email to reset your password (if your email
            address existed in our database).
        </InfoText>
        <InfoText>Please follow the link to continue.</InfoText>
    {:else}
        <InfoText>
            Please enter your email address. If we find it, we'll send you an
            email to reset your password.
        </InfoText>

        <form class="mt-4" on:submit={onSubmit}>
            <Label>
                Email address
                <Input
                    class="mt-2"
                    type="email"
                    bind:value={email}
                    disabled={loading}
                    required
                />
            </Label>

            <LoadingButton
                type="submit"
                buttonClass="mt-4"
                disabled={loading}
                {loading}>Send</LoadingButton
            >
        </form>
    {/if}
</AuthCard>
