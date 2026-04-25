<script lang="ts">
    import { Spinner } from "flowbite-svelte";
    import AuthCard from "../../layouts/AuthCard.svelte";
    import {
        getOrgSubdomain,
        getOrgSubdomainIDForAuth,
        performAuthCallback,
    } from "../../data/auth";
    import { navigate } from "../../router";
    import ErrorMsg from "../../components/ErrorMsg.svelte";
    import { humaniseAPIError } from "../../data/common";

    let error: string | null = $state(null);

    const urlParams = new URLSearchParams(location.search);
    const code = urlParams.get("code");
    const authState = urlParams.get("state");
    if (code === null || authState === null) {
        error = "Missing code or state in URL";
    } else {
        (async () => {
            const orgId = await getOrgSubdomainIDForAuth(getOrgSubdomain());
            if (!orgId) {
                error = "Could not find your organisation";
                return;
            }

            try {
                const userIsNew = await performAuthCallback(
                    orgId,
                    code,
                    authState
                );
                if (userIsNew) {
                    navigate(`/orgs/${orgId}/induction/member`);
                } else {
                    navigate(`/orgs/${orgId}`);
                }
            } catch (e) {
                error = humaniseAPIError(e);
            }
        })();
    }

    const onRetryClick = () => {
        navigate("/auth/login");
    };
</script>

<AuthCard>
    {#if error}
        <ErrorMsg e={error} retryable onretry={onRetryClick} />
    {:else}
        <Spinner class="mb-4" />
        <p>Loading...</p>
    {/if}
</AuthCard>
