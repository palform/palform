<script lang="ts">
    import { Label } from "flowbite-svelte";
    import PasswordPicker from "../../components/password/PasswordPicker.svelte";
    import InfoText from "../../components/type/InfoText.svelte";
    import AuthCard from "../../layouts/AuthCard.svelte";
    import LoadingButton from "../../components/LoadingButton.svelte";
    import { APIs } from "../../data/common";
    import { showFailureToast, showSuccessToast } from "../../data/toast";
    import { navigate } from "svelte-routing";

    export let verificationId: string;
    let newPassword = "";
    let loading = false;

    $: onResetClick = async (e: Event) => {
        e.preventDefault();
        loading = true;
        try {
            await APIs.passwordReset.userPasswordResetReset({
                verification_id: verificationId,
                new_password: newPassword,
            });
            await showSuccessToast(
                "Password changed successfully! Please sign in."
            );
            navigate("/auth/login");
        } catch (e) {
            await showFailureToast(e);
        }
        loading = false;
    };
</script>

<AuthCard title="Reset your password">
    <InfoText>Thanks for verifying your email address!</InfoText>
    <InfoText>To continue, please choose a new password.</InfoText>

    <form class="mt-4" on:submit={onResetClick}>
        <Label>
            New password
            <PasswordPicker
                class="mt-2"
                bind:value={newPassword}
                disabled={loading}
            />
        </Label>

        <LoadingButton
            buttonClass="mt-4"
            disabled={loading}
            {loading}
            type="submit"
        >
            Reset
        </LoadingButton>
    </form>
</AuthCard>
