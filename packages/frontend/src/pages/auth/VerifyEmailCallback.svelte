<script lang="ts">
    import { Spinner } from "flowbite-svelte";
    import InfoText from "../../components/type/InfoText.svelte";
    import AuthCard from "../../layouts/AuthCard.svelte";
    import { APIs } from "../../data/common";
    import { route, typedNavigate } from "../../router";
    import { showFailureToast, showSuccessToast } from "../../data/toast";

    let { verificationId } = route.getParams("/auth/verify/:verificationId");

    let loading = $state(true);
    $effect(() => {
        APIs.auth
            .authVerify(verificationId)
            .then(() => {
                typedNavigate("/auth/login", {
                    search: { create_initial_org: "true" },
                });
                showSuccessToast("Email verified! Please sign in.");
            })
            .catch((e) => {
                loading = false;
                showFailureToast(e);
            });
    });
</script>

<AuthCard title="Verifying your email...">
    {#if loading}
        <InfoText>Please wait...</InfoText>
        <Spinner class="mt-4" size="12" />
    {:else}
        <InfoText>
            Something went wrong. Please try following the verification link
            again.
        </InfoText>
    {/if}
</AuthCard>
