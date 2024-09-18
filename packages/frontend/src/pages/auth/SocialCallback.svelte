<script lang="ts">
    import { Spinner } from "flowbite-svelte";
    import InfoText from "../../components/type/InfoText.svelte";
    import {
        isSocialAuthProvider,
        processSocialAuthCallback,
        socialAuthServiceName,
    } from "../../data/auth/social";
    import AuthCard from "../../layouts/AuthCard.svelte";
    import { onMount } from "svelte";
    import { showFailureToast } from "../../data/toast";
    import { humaniseAPIError } from "../../data/common";
    import ErrorMsg from "../../components/ErrorMsg.svelte";
    import { navigate } from "svelte-routing";

    export let providerName: string;
    let error: undefined | string = undefined;

    onMount(() => {
        (async () => {
            if (!isSocialAuthProvider(providerName)) {
                await showFailureToast(`Provider '${providerName}' not found`);
                return;
            }

            try {
                const newOrgID = await processSocialAuthCallback(providerName);

                if (newOrgID) {
                    navigate(`/orgs/${newOrgID}/induction/billing`);
                } else {
                    navigate("/");
                }
            } catch (e) {
                error = humaniseAPIError(e);
            }
        })();
    });

    const onRetry = () => {
        navigate("/auth/login");
    };
</script>

{#if isSocialAuthProvider(providerName)}
    <AuthCard
        title={`Signing you in with ${socialAuthServiceName(providerName)}`}
    >
        {#if error}
            <ErrorMsg e={error} retryable on:retry={onRetry} />
        {:else}
            <Spinner class="mb-4" />
            <InfoText>Please wait...</InfoText>
        {/if}
    </AuthCard>
{/if}
