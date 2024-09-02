<script lang="ts">
    import { Spinner } from "flowbite-svelte";
    import InfoText from "../../components/type/InfoText.svelte";
    import AuthCard from "../../layouts/AuthCard.svelte";
    import { APIs } from "../../data/common";
    import { navigate } from "svelte-routing";
    import { showFailureToast, showSuccessToast } from "../../data/toast";

    export let verificationId: string;

    let loading = true;
    APIs.auth
        .authVerify(verificationId)
        .then(() => {
            navigate("/auth/login?create_initial_org");
            showSuccessToast("Email verified! Please sign in.");
        })
        .catch((e) => {
            loading = false;
            showFailureToast(e);
        });
</script>

<AuthCard title="Verifying your email...">
    {#if loading}
        <InfoText>Please wait...</InfoText>
        <Spinner class="mt-4" size={12} />
    {:else}
        <InfoText>
            Something went wrong. Please try following the verification link
            again.
        </InfoText>
    {/if}
</AuthCard>
