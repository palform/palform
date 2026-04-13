<script>
    import { Button } from "flowbite-svelte";
    import CardGrid from "../../../components/CardGrid.svelte";
    import InductionStepCard from "../../../components/induction/InductionStepCard.svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { p } from "../../../router";

    const orgCtx = getOrgContext();
</script>

<CardGrid>
    <InductionStepCard title="Your brand, front and centre">
        Use a custom subdomain to generate links for your forms: so short they
        can fit on posters and in QR codes. Your brand will clearly be in every
        interaction made by your customers.
    </InductionStepCard>
    <InductionStepCard title="Zero set-up time">
        Simply choose a subdomain and individually generate short links for your
        forms (starting from 1 character!): choose your own or let us generate
        them.
    </InductionStepCard>
    <InductionStepCard title="Get started now">
        Subdomains are available exclusively on our upgraded plans. Get started
        now with fair plans that you can cancel whenever you want.

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
