<script lang="ts">
    import type { APIUserKeyWithIdentity } from "@paltiverse/palform-typescript-openapi";
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { APIs } from "../../data/common";
    import { showFailureToast } from "../../data/toast";
    import { isEntitled } from "../../data/billing/entitlement";
    import OrganisationKeyBrowser from "../../components/orgs/keys/OrganisationKeyBrowser.svelte";
    import CardGrid from "../../components/CardGrid.svelte";
    import InductionStepCard from "../../components/induction/InductionStepCard.svelte";
    import { Button } from "flowbite-svelte";
    import { navigate } from "svelte-routing";

    const orgCtx = getOrgContext();
    const entitled = isEntitled("crypto_details");
    let loading = true;
    let keys: APIUserKeyWithIdentity[] | null = null;

    APIs.orgKeys()
        .then((a) => a.orgKeysList($orgCtx.org.id))
        .then((resp) => {
            keys = resp.data;
            loading = false;
        })
        .catch(showFailureToast);

    $: onContinueClick = () => {
        navigate(`/orgs/${$orgCtx.org.id}/settings/billing`);
    };
</script>

{#if !$entitled}
    <CardGrid>
        <InductionStepCard title="All keys, one page">
            Get a crystal-clear view of all the encryption keys in your
            organisation right here in the dashboard.
        </InductionStepCard>
        <InductionStepCard title="Admin super-powers">
            Delete keys belonging to any user in just a single click, preventing
            future responses from being encrypted with it.
        </InductionStepCard>
        <InductionStepCard title="Get started">
            To access this page, please upgrade your plan.
            <Button class="block mt-2" on:click={onContinueClick}>
                Continue
            </Button>
        </InductionStepCard>
    </CardGrid>
{:else}
    <OrganisationKeyBrowser />
{/if}
