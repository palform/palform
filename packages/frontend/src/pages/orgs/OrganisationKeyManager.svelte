<script lang="ts">
    import { getOrgContext } from "../../data/contexts/orgLayout";
    import { isEntitled } from "../../data/billing/entitlement";
    import OrganisationKeyBrowser from "../../components/orgs/keys/OrganisationKeyBrowser.svelte";
    import CardGrid from "../../components/CardGrid.svelte";
    import InductionStepCard from "../../components/induction/InductionStepCard.svelte";
    import { Button } from "flowbite-svelte";
    import { p } from "../../router";

    const orgCtx = getOrgContext();
    const entitled = isEntitled("crypto_details");
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
            {#snippet footer()}
                <Button
                    class="mt-3"
                    href={p("/orgs/:orgId/settings/billing", {
                        params: { orgId: $orgCtx.org.id },
                    })}
                >
                    Continue
                </Button>
            {/snippet}
        </InductionStepCard>
    </CardGrid>
{:else}
    <OrganisationKeyBrowser />
{/if}
