<script lang="ts">
    import { navigate } from "svelte-routing";
    import CardGrid from "../../../components/CardGrid.svelte";
    import InductionStepCard from "../../../components/induction/InductionStepCard.svelte";
    import { getOrgContext } from "../../../data/contexts/orgLayout";
    import { Button } from "flowbite-svelte";

    const orgCtx = getOrgContext();
    $: onGetStartedClick = () =>
        navigate(`/orgs/${$orgCtx.org.id}/settings/billing`);
</script>

<CardGrid>
    <InductionStepCard title="Centralised authentication support">
        Connect to your existing authentication provider using OpenID Connect.
        Automatically provision new users and even assign them into teams based
        on group membership.
    </InductionStepCard>
    <InductionStepCard title="Easy, spec-compliant setup">
        Our authentication strongly conforms to the OpenID Connect
        specification, using OIDC Discovery to learn information about your
        system, saving you having to enter it all in.
    </InductionStepCard>
    <InductionStepCard title="Get started in 10 minutes">
        OpenID Connect authentication is available exclusively on our upgraded
        plans.

        <Button class="block mt-2" on:click={onGetStartedClick}>
            Get started
        </Button>
    </InductionStepCard>
</CardGrid>
