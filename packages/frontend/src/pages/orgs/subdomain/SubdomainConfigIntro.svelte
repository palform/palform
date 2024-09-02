<script>
    import { Button } from "flowbite-svelte";
    import CardGrid from "../../../components/CardGrid.svelte";
    import InductionStepCard from "../../../components/induction/InductionStepCard.svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { navigate } from "svelte-routing";

    const orgCtx = getOrgContext();
    $: onGetStartedClick = () =>
        navigate(`/orgs/${$orgCtx.org.id}/settings/billing`);
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

        <Button class="mt-3 block" on:click={onGetStartedClick}>
            Continue
        </Button>
    </InductionStepCard>
</CardGrid>
